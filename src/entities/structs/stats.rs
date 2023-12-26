use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityStat {
    Thirst(f32),
    Hunger(f32),
    Energy(f32),
    Health(f32)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityStats {
    pub thirst: f32,
    pub hunger: f32,
    pub energy: f32,
    pub health: f32
}

impl Default for EntityStats {
    fn default() -> Self {
        Self {
            thirst: Default::default(),
            hunger: Default::default(),
            energy: 1.0,
            health: 100.0
        }
    }
}

impl EntityStats {
    pub fn get_formatted(&self) -> String {
        format!(
            "Thirst: {}\nHunger: {}\nEnergy: {}",
            self.thirst,
            self.hunger,
            self.energy
        )
    }
}