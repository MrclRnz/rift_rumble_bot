use crate::entities::entity::RiftRumbleEntity;
use crate::entities::entity::GenericStaticData;
use super::entity::RiftRumbleEntityCollection;

use std::collections::HashMap;
use std::fs;
use std::io;
use serde_json::Value;

const DATA_DRAGON_ROOT: &str = "./resources/11.23.1/data/en_US";
const SUMMONERS_RIFT_ID: &str = "11";
const CLASSIC_MODE: &str = "CLASSIC";

pub fn load_static_data(rift_rumble_entity_collection: &mut RiftRumbleEntityCollection) -> Result<(), io::Error> {
    //Champions
    let paths = fs::read_dir(DATA_DRAGON_ROOT.to_owned() + "/champion/")?;
    for path in paths {
        let path = path?.path();
        if let true = path.is_file() {
            load_champion_from_file(&mut rift_rumble_entity_collection.champions, path.to_str().ok_or(io::ErrorKind::InvalidInput)?)
                .ok_or(io::ErrorKind::InvalidData)?;
        }
    }

    //Items
    let paths = DATA_DRAGON_ROOT.to_owned() + "/item.json";
    load_items_from_file(&mut rift_rumble_entity_collection.items, paths.as_str()).ok_or(io::ErrorKind::InvalidData)?;

    //Runes
    let paths = DATA_DRAGON_ROOT.to_owned() + "/runesReforged.json";
    load_runes_from_file(&mut rift_rumble_entity_collection.runes, paths.as_str()).ok_or(io::ErrorKind::InvalidData)?;

    //Summoners
    let paths = DATA_DRAGON_ROOT.to_owned() + "/summoner.json";
    load_summoners_from_file(&mut rift_rumble_entity_collection.summoners, paths.as_str()).ok_or(io::ErrorKind::InvalidData)?;

    Ok(())
}

fn load_champion_from_file(champions: &mut Vec<RiftRumbleEntity>, file_path: &str) -> Option<()> {
    let file = fs::File::open(file_path).ok()?;
    let json: Value = serde_json::from_reader(file).ok()?;
    let json = json.get("data")?.as_object()?;

    let json = json.get(json.keys().next()?)?.as_object()?;

    let id = json.get("key")?.as_str()?.to_owned();
    let name = json.get("name")?.as_str()?.to_owned();

    champions.push(RiftRumbleEntity::Champion(GenericStaticData { id, name }));

    Some(())
}

fn load_runes_from_file(runes: &mut HashMap<String, HashMap<u8, Vec<RiftRumbleEntity>>>, file_path: &str) -> Option<()> {
    let file = fs::File::open(file_path).ok()?;
    let json: Value = serde_json::from_reader(file).ok()?;
    for entry in json.as_array()? {
        let object = entry.as_object()?;
        let tree = object.get("key")?.as_str()?.to_owned();
        for (slot_number, content) in object.get("slots")?.as_array()?.iter().enumerate() {
            for rune in content.get("runes")?.as_array()? {
                let id = rune.get("id")?.as_u64()?.to_string();
                let name = rune.get("key")?.as_str()?.to_owned();

                let tree = runes.entry(tree.to_owned()).or_insert(HashMap::new());
                let slot_row = tree.entry(slot_number as u8).or_insert(Vec::new());

                slot_row.push(RiftRumbleEntity::Rune(GenericStaticData {
                    id,
                    name,
                }));
            }
        }
    }
    Some(())
}

fn load_items_from_file(items: &mut Vec<RiftRumbleEntity>, file_path: &str) -> Option<()> {
    let file = fs::File::open(file_path).ok()?;
    let json: Value = serde_json::from_reader(file).ok()?;
    let json = json.get("data")?.as_object()?;

    for id in json.keys() {
        let item = json.get(id)?;
        if item_is_eligible(item)? {
            let name = item.get("name")?.as_str()?.to_owned();

            items.push(RiftRumbleEntity::Item(GenericStaticData {
                id: id.to_owned(),
                name,
            }));
        }
    }
    Some(())
}

fn load_summoners_from_file(summoners: &mut Vec<RiftRumbleEntity>, file_path: &str) -> Option<()> {
    let file = fs::File::open(file_path).ok()?;
    let json: Value = serde_json::from_reader(file).ok()?;
    let json = json.get("data")?.as_object()?;

    for id in json.keys() {
        let summoner = json.get(id)?;
        if summoner
            .get("modes")?
            .as_array()?
            .contains(&Value::String(String::from(CLASSIC_MODE)))
        {
            let name = summoner.get("name")?.as_str()?.to_owned();
            summoners.push(RiftRumbleEntity::Summoner(GenericStaticData {
                id: id.to_owned(),
                name,
            }));
        }
    }
    Some(())
}

fn item_is_eligible(item: &Value) -> Option<bool> {
    if item.get("into").is_none() {
        if item.get("gold")?.get("purchasable")?.as_bool()? {
            if item.get("gold")?.get("total")?.as_u64()? > 1700 {
                if item.get("maps")?.get(SUMMONERS_RIFT_ID)?.as_bool()? {
                    if !item
                        .get("tags")?
                        .as_array()?
                        .contains(&Value::String(String::from("Consumable")))
                    {
                        return Some(true);
                    }
                }
            }
        }
    }
    Some(false)
}
