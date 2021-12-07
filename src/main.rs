use rift_rumble_bot::entities::{entity::RiftRumbleEntityCollection, entity_loader};
use rift_rumble_bot::discord::bot;
fn main() {
    let mut entity_collection = RiftRumbleEntityCollection::new();
    if let Err(e) = entity_loader::load_static_data(&mut entity_collection) {
        panic!(
            "Error while loading all static data from data dragon: {}",
            e
        );
    }

    let champion = entity_collection.randomize_champion();
    let items = entity_collection.randomize_items();
    let runes = entity_collection.randomize_runes();
    let summoners = entity_collection.randomize_summoners();

    println!("Champion: {:?}", champion);
    println!("Items: {:?}", items);
    println!("Runes: {:?}", runes);
    println!("Summoners: {:?}", summoners);

    bot::init_bot();
}

