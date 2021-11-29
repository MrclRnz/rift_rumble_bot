use rand::{distributions::uniform::SampleRange, Rng};
use std::{collections::HashMap, ops::{RangeToInclusive, RangeInclusive}, slice::SliceIndex};

#[derive(Debug)]
pub struct GenericStaticData {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub enum RiftRumbleEntity {
    Champion(GenericStaticData),
    Item(GenericStaticData),
    Rune(GenericStaticData),
    Summoner(GenericStaticData),
}

pub struct RiftRumbleEntityCollection {
    pub champions: Vec<RiftRumbleEntity>,
    pub items: Vec<RiftRumbleEntity>,
    pub runes: HashMap<String, HashMap<u8, Vec<RiftRumbleEntity>>>,
    pub summoners: Vec<RiftRumbleEntity>,
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

    pub fn randomize_champion(&self) -> &RiftRumbleEntity {
        let mut rng = rand::thread_rng();
        self.champions
            .get(rng.gen_range(0..=self.champions.len()))
            .unwrap()
    }

    pub fn randomize_items(&self) -> Vec<&RiftRumbleEntity> {
        let mut selected_items: HashMap<String, &RiftRumbleEntity> = HashMap::new();
        loop {
            let mut rng = rand::thread_rng();
            let item = self.items.get(rng.gen_range(0..=self.items.len())).unwrap();
            if let RiftRumbleEntity::Item(i) = item {
                selected_items.entry(i.id.to_owned()).or_insert(item);
            }
            if selected_items.len() == 5 {
                break;
            }
        }
        selected_items.into_values().collect()
    }

    pub fn randomize_runes(&self) -> HashMap<String, Vec<&RiftRumbleEntity>> {
        let mut selected_runes: HashMap<String, Vec<&RiftRumbleEntity>> = HashMap::new();
        let mut rng = rand::thread_rng();
        let tree_index = rng.gen_range(0..=self.runes.len());
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
            tree_vector.push(entry.1.get(rng.gen_range(0..entry.1.len())).unwrap());
        }

        let mut rng = rand::thread_rng();
        let mut second_tree_index = rng.gen_range(0..=self.runes.len());
        while tree_index == second_tree_index {
            let mut rng = rand::thread_rng();
            second_tree_index = rng.gen_range(0..=self.runes.len());
        }
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

        let mut rng = rand::thread_rng();
        let first_random_row = rng.gen_range(1..=3);
        let mut rng = rand::thread_rng();
        let mut second_random_row = rng.gen_range(1..=3);
        while first_random_row == second_random_row {
            let mut rng = rand::thread_rng();
            second_random_row = rng.gen_range(1..=3);
        }

        selected_runes
    }

    pub fn randomize_summoners(&self) -> (&RiftRumbleEntity, &RiftRumbleEntity) {
        let first_index = get_next_random_num(None, 0..=self.summoners.len());
        let mut second_index = get_next_random_num(first_index, 0..=self.summoners.len());


        let first_summoner = self.summoners.get(first_index).unwrap();
        let second_summoner = self.summoners.get(second_index).unwrap();

        (first_summoner, second_summoner)
    }
}

fn get_next_random_num<T: SampleUniform>(already_used: Option<i32>, range: RangeInclusive<T>) -> i32 {
    let mut rng = rand::thread_rng();
    let x = 1..=2;
    let mut num = rng.gen_range(range);
    if let Some(u) = already_used {
        while num == u {
            num = rng.gen_range(range);
        }
    }
    num
}
