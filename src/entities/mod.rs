use serde::{Serialize, Deserialize};

mod human;
mod player;
mod structs;

pub use human::Human;
pub use player::Player;
pub use structs::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityType {
    Human
}

pub trait Entity {
    fn get_entity_save_data(&self) -> String;
}