use serde::{Serialize, Deserialize};
use std::io;

use crate::util::{collect_with_options, save_to_file_bin, load_from_file_bin};
use crate::entities::{
    EntityAttribute,
    Human
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub human: Human
}

impl Player {
    const SAVE_PATH: &'static str = "saved/player.dat";

    pub fn new(first_name: Option<&str>, last_name: Option<&str>, age: Option<u8>) -> Self {
        let mut human = Human::new();
        
        if let Some(first_name) = first_name {
            human.set_first_name(first_name);
        }

        if let Some(last_name) = last_name {
            human.set_last_name(last_name);
        }

        if let Some(age) = age {
            human.set_age(age);
        }

        Self { human }
    }

    pub fn load() -> Result<Self, &'static str> {
        let file = load_from_file_bin(Self::SAVE_PATH)
            .expect("Failed to load player file");

        // There is no data stored yet
        // Create a new one and return here
        if file.len() <= 0 {
            let player = Self::init();
            let _ = player.save();
            return Ok(player);
        }

        let player: Self = bincode::deserialize(&file)
            .expect("Failed to load player from file");

        Ok(player)        
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let serialized = bincode::serialize(self)
            .expect("Failed to serialize player");

        save_to_file_bin(Self::SAVE_PATH, &serialized)
    }

    pub fn init() -> Self {
        // It begins...
        let mut pl = Self::new(None,None, None);

        let stat_gain = 3;
        let prompt = collect_with_options(
            "You wake up after a long slumber...\n\
            You begin to wonder a lot of things about yourself...\n\n\
            What attribute best defines you?", 
            &[
                &format!("Thinker (+{} Intelligence)", stat_gain),
                &format!("Hard worker (+{} Strength)", stat_gain),
                &format!("Socialite (+{} Charisma)", stat_gain),
                &format!("Fortunate (+{} Luck)", stat_gain),
                &format!("Fighter (+{} Combat)", stat_gain)
            ]
        );

        match prompt.unwrap() {
            0 => pl.human.attributes.add_attribute(EntityAttribute::Intelligence(stat_gain)),
            1 => pl.human.attributes.add_attribute(EntityAttribute::Strength(stat_gain)),
            2 => pl.human.attributes.add_attribute(EntityAttribute::Charisma(stat_gain)),
            3 => pl.human.attributes.add_attribute(EntityAttribute::Luck(stat_gain)),
            4 => pl.human.attributes.add_attribute(EntityAttribute::Combat(stat_gain)),
            _ => unreachable!()
        };

        println!(
            "Your name is {}\n\
            You are {} years old\n\n\
            These are your attributes:\n{}\n\n\
            These are your stats:\n{}",
            pl.human.get_name_formatted(),
            pl.human.age,
            pl.human.get_attributes_formatted(),
            pl.human.get_stats_formatted()
        );

        pl
    }

    pub fn print_menu(&self) {
        let prompt = collect_with_options(
            "", 
            &[
               "Display Stats",
               "Display Attributes",
               "Inventory",
               "Continue",
               "Exit"
            ]
        );

        match prompt.unwrap() {
            0 => { self.human.stats.get_formatted(); },
            1 => { self.human.attributes.get_formatted(); },
            2 => { println!("{:?}", self.human.inventory); },
            3 => todo!(),
            4 => todo!(),
            _ => unreachable!()
        };
    }
}