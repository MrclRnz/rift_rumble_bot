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
    
    bot::init_bot(entity_collection);
}

