use rand::{prelude::SliceRandom, thread_rng, Rng};
use serenity::model::prelude::UserId;
use serenity::prelude::TypeMapKey;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct GenericStaticData {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub enum RiftRumbleEntity {
    Champion(GenericStaticData),
    Item(GenericStaticData, bool),
    Rune(GenericStaticData),
    Summoner(GenericStaticData),
}
#[derive(Debug)]
pub struct RiftRumbleEntitySet {
    pub champion: Arc<RiftRumbleEntity>,
    pub items: Vec<Arc<RiftRumbleEntity>>,
    pub runes: HashMap<String, Vec<Arc<RiftRumbleEntity>>>,
    pub summoners: (Arc<RiftRumbleEntity>, Arc<RiftRumbleEntity>),
    pub skill_order: (String, String, String),
}

pub struct RiftRumbleEntityCollection {
    pub champions: Vec<Arc<RiftRumbleEntity>>,
    pub items: Vec<Arc<RiftRumbleEntity>>,
    pub runes: HashMap<String, HashMap<u8, Vec<Arc<RiftRumbleEntity>>>>,
    pub summoners: Vec<Arc<RiftRumbleEntity>>,
}
impl TypeMapKey for RiftRumbleEntityCollection {
    type Value = Arc<RiftRumbleEntityCollection>;
}

#[derive(Debug)]
pub struct CustomHashMap(pub HashMap<UserId, RiftRumbleEntitySet>);
impl TypeMapKey for CustomHashMap {
    type Value = Arc<RwLock<CustomHashMap>>;
}

fn randomize_skill_order() -> (String, String, String) {
    let mut skills = vec!["Q", "W", "E"];
    skills.shuffle(&mut thread_rng());
    (
        skills[0].to_owned(),
        skills[1].to_owned(),
        skills[2].to_owned(),
    )
}

impl RiftRumbleEntityCollection {
    pub fn new() -> RiftRumbleEntityCollection {
        RiftRumbleEntityCollection {
            champions: Vec::new(),
            items: Vec::new(),
            runes: HashMap::new(),
            summoners: Vec::new(),
        }
    }

    pub fn randomize_set(&self) -> RiftRumbleEntitySet {
        RiftRumbleEntitySet {
            champion: self.randomize_champion(),
            items: self.randomize_items(),
            runes: self.randomize_runes(),
            summoners: self.randomize_summoners(),
            skill_order: randomize_skill_order(),
        }
    }

    fn randomize_champion(&self) -> Arc<RiftRumbleEntity> {
        self.champions
            .get(get_next_random_num_closed(None, 0, self.champions.len()))
            .unwrap()
            .clone()
    }

    fn randomize_items(&self) -> Vec<Arc<RiftRumbleEntity>> {
        let mut selected_items: HashMap<String, Arc<RiftRumbleEntity>> = HashMap::new();

        for item in self.items.iter() {
            if let RiftRumbleEntity::Item(i, is_mythic) = item.as_ref() {
                if *is_mythic {
                    selected_items
                        .entry(i.id.to_owned())
                        .or_insert(item.clone());
                    break;
                }
            }
        }
        loop {
            let item = self
                .items
                .get(get_next_random_num_closed(None, 0, self.items.len()))
                .unwrap()
                .clone();
            if let RiftRumbleEntity::Item(i, is_mythic) = item.as_ref() {
                if !*is_mythic {
                    selected_items.entry(i.id.to_owned()).or_insert(item);
                }
            }
            if selected_items.len() == 5 {
                break;
            }
        }
        selected_items.into_values().collect()
    }

    fn randomize_runes(&self) -> HashMap<String, Vec<Arc<RiftRumbleEntity>>> {
        let mut selected_runes: HashMap<String, Vec<Arc<RiftRumbleEntity>>> = HashMap::new();
        let mut rng = rand::thread_rng();
        let tree_index = get_next_random_num_closed(None, 0, self.runes.len());
        let tree = self
            .runes
            .keys()
            .enumerate()
            .filter(|i| i.0 == tree_index)
            .find(|_| true)
            .unwrap()
            .1;
        selected_runes.insert(tree.to_owned(), Vec::new());
        let tree_vector = selected_runes.get_mut(tree).unwrap();
        for entry in self.runes.get(tree).unwrap().iter() {
            tree_vector.push(
                entry
                    .1
                    .get(rng.gen_range(0..entry.1.len()))
                    .unwrap()
                    .clone(),
            );
        }

        let second_tree_index = get_next_random_num_closed(Some(tree_index), 0, self.runes.len());
        let tree = self
            .runes
            .keys()
            .enumerate()
            .filter(|i| i.0 == second_tree_index)
            .find(|_| true)
            .unwrap()
            .1;
        selected_runes.insert(tree.to_owned(), Vec::new());
        let tree_vector = selected_runes.get_mut(tree).unwrap();

        let first_random_row = get_next_random_num_closed(None, 1, 3) as u8;
        let second_random_row =
            get_next_random_num_closed(Some(first_random_row as usize), 1, 3) as u8;
        for entry in self.runes.get(tree).unwrap().iter() {
            if *entry.0 == first_random_row || *entry.0 == second_random_row {
                tree_vector.push(
                    entry
                        .1
                        .get(rng.gen_range(0..entry.1.len()))
                        .unwrap()
                        .clone(),
                );
            }
        }

        selected_runes
    }

    fn randomize_summoners(&self) -> (Arc<RiftRumbleEntity>, Arc<RiftRumbleEntity>) {
        let first_index = get_next_random_num_closed(None, 0, self.summoners.len());
        let second_index = get_next_random_num_closed(Some(first_index), 0, self.summoners.len());
        let first_summoner = self.summoners.get(first_index).unwrap().clone();
        let second_summoner = self.summoners.get(second_index).unwrap().clone();

        (first_summoner, second_summoner)
    }
}

fn get_next_random_num_closed(taken: Option<usize>, start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    let mut number = rng.gen_range(start..=end - 1);
    if let Some(t) = taken {
        while number == t {
            number = rng.gen_range(start..=end - 1);
        }
    }
    number
}
