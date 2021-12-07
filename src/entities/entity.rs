use rand::Rng;
use std::collections::HashMap;

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
        self.champions
            .get(get_next_random_num_closed(None, 0, self.champions.len()))
            .unwrap()
    }

    pub fn randomize_items(&self) -> Vec<&RiftRumbleEntity> {
        let mut selected_items: HashMap<String, &RiftRumbleEntity> = HashMap::new();
        loop {
            let item = self
                .items
                .get(get_next_random_num_closed(None, 0, self.items.len()))
                .unwrap();
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
            tree_vector.push(entry.1.get(rng.gen_range(0..entry.1.len())).unwrap());
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
                tree_vector.push(entry.1.get(rng.gen_range(0..entry.1.len())).unwrap());
            }
        }

        selected_runes
    }

    pub fn randomize_summoners(&self) -> (&RiftRumbleEntity, &RiftRumbleEntity) { 
        let first_index = get_next_random_num_closed(None, 0, self.summoners.len());
        let second_index = get_next_random_num_closed(Some(first_index), 0, self.summoners.len());
        let first_summoner = self.summoners.get(first_index).unwrap();
        let second_summoner = self.summoners.get(second_index).unwrap();

        (first_summoner, second_summoner)
    }
}

fn get_next_random_num_closed(taken: Option<usize>, start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    let mut number = rng.gen_range(start..=end-1);
    if let Some(t) = taken {
        while number == t {
            number = rng.gen_range(start..=end-1);
        }
    }
    number
}
