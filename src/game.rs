use serde::{Serialize, Deserialize};
use bincode;
use std::{
    fs,
    io,
    time::{Duration, UNIX_EPOCH, SystemTime}
};

use crate::entities::Player;
use crate::util::{
    folder_exists,
    load_from_file_bin,
    save_to_file_bin
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub time: SystemTime,
}

impl Game {
    // Tue Jan 01 1985 05:00:00 GMT+0000
    const GAME_START_EPOCH: u64 = 1672549200;
    const GAME_START_DURATION: Duration = Duration::from_secs(Self::GAME_START_EPOCH);
    const GAME_FILEPATH: &'static str = "saved/game.dat";
    const _COUNTRY: &'static str = "USA";
    const _CITY: &'static str = "New York";

    pub fn new() -> Self {
        let player = Player::init();
        let _ = player.save();

        let game = Self { time: UNIX_EPOCH + Self::GAME_START_DURATION };
        let _ = game.save();

        return game;
    }

    pub fn init() {
        const SAVE_FOLDER: &str = "saved";
        let game;

        // If the save folder doesnt exist, create it
        if !folder_exists(SAVE_FOLDER) {
            if !fs::metadata(SAVE_FOLDER).is_ok() {
                fs::create_dir(SAVE_FOLDER)
                    .expect("Failed to create save folder");
            
                game = Self::new();
            } else {
                game = Self::load().ok().unwrap();
            }
        } else {
            game = Self::load().ok().unwrap();
        }

        let player = Player::load().unwrap_or(Player::init());

        loop {
            game.tick(&player);
        }
    }

    fn tick(&self, player: &Player) {
        player.print_menu();
    }

    pub fn load() -> Result<Self, &'static str>  {
        let file = load_from_file_bin(Self::GAME_FILEPATH)
            .expect("Failed to load game file");

        // There is no data stored yet
        // Create a new one and return here
        if file.len() <= 0 {
            let game = Self { time: UNIX_EPOCH + Self::GAME_START_DURATION };
            let _ = game.save();
            return Ok(game);
        }

        let game: Self = bincode::deserialize(&file)
            .expect("Failed to load game from file");

        Ok(game)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let serialized = bincode::serialize(self)
            .expect("Failed to serialize game");

        save_to_file_bin(Self::GAME_FILEPATH, &serialized)
    }

    // Save gamestate
    pub fn exit() {

    }
}