use serde::{Serialize, Deserialize};

use crate::util::random_range;

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityAttribute {
    Intelligence(u8),
    Strength(u8),
    Charisma(u8),
    Luck(u8),
    Combat(u8)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityAttributes {
    pub intelligence: u8,
    pub strength: u8,
    pub charisma: u8,
    pub luck: u8,
    pub combat: u8,
}

impl Default for EntityAttributes {
    fn default() -> Self {
        Self {
            intelligence: Default::default(),
            strength: Default::default(),
            charisma: Default::default(),
            luck: Default::default(),
            combat: Default::default()
        }
    }
}

impl EntityAttributes {
    pub fn initial_stats() -> Self {
        let get_initial_stat = || -> u8 {
            random_range(0.0..=5.0) as u8
        };

        Self {
            intelligence: get_initial_stat(),
            strength: get_initial_stat(),
            charisma: get_initial_stat(),
            luck: get_initial_stat(),
            combat: get_initial_stat(),
        }
    }

    pub fn get_formatted(&self) -> String {
        format!(
            "Intelligence: {}\nStrength: {}\nCharisma: {}\nLuck: {}\nCombat: {}",
            self.intelligence,
            self.strength,
            self.charisma,
            self.luck,
            self.combat
        )
    }

    /* Attribute setters|getters */
    pub fn get_attribute(&self, attr: EntityAttribute) -> &u8 {
        match attr {
            EntityAttribute::Intelligence(_) => &self.intelligence,
            EntityAttribute::Strength(_) => &self.strength,
            EntityAttribute::Charisma(_) => &self.charisma,
            EntityAttribute::Luck(_) => &self.luck,
            EntityAttribute::Combat(_) => &self.combat,
        }
    }

    pub fn get_attribute_mut(&mut self, attr: EntityAttribute) -> &mut u8 {
        match attr {
            EntityAttribute::Intelligence(_) => &mut self.intelligence,
            EntityAttribute::Strength(_) => &mut self.strength,
            EntityAttribute::Charisma(_) => &mut self.charisma,
            EntityAttribute::Luck(_) => &mut self.luck,
            EntityAttribute::Combat(_) => &mut self.combat,
        }
    }

    pub fn set_attribute(&mut self, attr: EntityAttribute) {
        match attr {
            EntityAttribute::Intelligence(value) => self.intelligence = value,
            EntityAttribute::Strength(value) => self.strength = value,
            EntityAttribute::Charisma(value) => self.charisma = value,
            EntityAttribute::Luck(value) => self.luck = value,
            EntityAttribute::Combat(value) => self.combat = value,
        }
    }

    pub fn add_attribute(&mut self, attr: EntityAttribute) {
        match attr {
            EntityAttribute::Intelligence(value) => self.intelligence.checked_add(value),
            EntityAttribute::Strength(value) => self.strength.checked_add(value),
            EntityAttribute::Charisma(value) => self.charisma.checked_add(value),
            EntityAttribute::Luck(value) => self.luck.checked_add(value),
            EntityAttribute::Combat(value) => self.combat.checked_add(value),
        };
    }

    pub fn sub_attribute(&mut self, attr: EntityAttribute) {
        match attr {
            EntityAttribute::Intelligence(value) => self.intelligence.checked_sub(value),
            EntityAttribute::Strength(value) => self.strength.checked_sub(value),
            EntityAttribute::Charisma(value) => self.charisma.checked_sub(value),
            EntityAttribute::Luck(value) => self.luck.checked_sub(value),
            EntityAttribute::Combat(value) => self.combat.checked_sub(value),
        };
    }
}