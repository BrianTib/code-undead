use serde::{Serialize, Deserialize};

use crate::util::{random, random_range, read_file_lines};
use super::{
    EntityType,
    EntityAttributes,
    EntityStats
};

// Placeholder types for Item, Relationship, Quest, and Emotion
#[derive(Serialize, Deserialize, Debug)]
pub struct Item;

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship;

#[derive(Serialize, Deserialize, Debug)]
pub enum Emotion {
    Happinness(f32),
    Anger(f32),
    Surprise(f32),
    Excitement(f32),
    Fear(f32),
    Disgust(f32),
    Neutral
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Gender {
    Male,
    Female
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Human {
    /// Whether or not the entity is an undead
    pub undead: bool,
    /* Identifiable information about this human */ 
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub gender: Gender,

    pub attributes: EntityAttributes,
    pub stats: EntityStats,

    // TODO: Add diseases
    pub inventory: Vec<Item>,
    pub relationships: Vec<Relationship>,
    pub current_location: String,
    pub mood: Vec<Emotion>,
    entity_type: EntityType,
}

impl Human {
    pub fn new() -> Self {
        let gender = if random() >= 0.5 { Gender::Male } else { Gender::Female };

        Self {
            age: random_range(17.0..=30.0) as u8,
            attributes: EntityAttributes::initial_stats(),
            current_location: String::default(),
            entity_type: EntityType::Human,
            first_name: get_random_first_name(&gender),
            gender,
            inventory: Vec::new(),
            last_name: get_random_last_name(),
            mood: Vec::new(),
            relationships: Vec::new(),
            stats: EntityStats::default(),
            undead: false
        }
    }

    pub fn set_first_name(&mut self, name: &str) -> &mut Self {
        self.first_name = name.to_string();
        self
    }

    pub fn set_last_name(&mut self, name: &str) -> &mut Self {
        self.last_name = name.to_string();
        self
    }

    pub fn set_age(&mut self, age: u8) -> &mut Self {
        self.age = age;
        self
    }

    pub fn get_name_formatted(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn get_attributes_formatted(&self) -> String {
        self.attributes.get_formatted()
    }

    pub fn get_stats_formatted(&self) -> String {
        self.stats.get_formatted()
    }
}

fn get_random_first_name(gender: &Gender) -> String {
    let gender = match gender {
        Gender::Male => "male",
        Gender::Female => "female"
    };

    match read_file_lines(&format!("./random/first-names-{}.txt", gender)) {
        Ok(lines) => {
            let index = (random() * lines.len() as f32) as usize;

            if let Some(name) = lines.get(index) {
                return name.to_owned();
            }
        },
        Err(_) => {},
    }

    String::new()
}

fn get_random_last_name() -> String {
    match read_file_lines("./random/last-names.txt") {
        Ok(lines) => {
            let index = random_range(0.0..=lines.len() as f32) as usize;

            if let Some(name) = lines.get(index) {
                return name.to_owned();
            }
        },
        Err(_) => {},
    }

    String::new()
}