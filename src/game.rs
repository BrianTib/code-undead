use serde::{Serialize, Deserialize};
use bincode;
use std::{
    fs,
    io,
    sync::{Arc, Mutex},
    time::{Duration, UNIX_EPOCH, SystemTime}
};

use crate::entities::Player;
use crate::util::{
    folder_exists,
    load_from_file_bin,
    save_to_file_bin
};

// Tue Jan 01 1985 05:00:00 GMT+0000
const GAME_START_EPOCH: u64 = 1672549200;
const GAME_START_DURATION: Duration = Duration::from_secs(GAME_START_EPOCH);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Game {
    pub time: SystemTime,
}

// pub enum GameTickSignal {
//     Continue,
//     Exit
// }

impl Default for Game {
    fn default() -> Self {
        Self { time: UNIX_EPOCH + GAME_START_DURATION }
    }
}

impl Game {
    const GAME_FILEPATH: &'static str = "saved/game.dat";
    const _COUNTRY: &'static str = "USA";
    const _CITY: &'static str = "New York";

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

        let game_arc = Arc::new(Mutex::new(game));
        let player = Player::init(Arc::clone(&game_arc));
        
        let game = Arc::clone(&game_arc);

        loop {
            let game = game.lock().unwrap();
            let shutdown_signal = game.tick(&player);
            if shutdown_signal {
                Self::exit_gracefully();
                break;
            }
        }
    }

    fn tick(&self, player: &Player) -> bool {
        player.print_menu()
    }

    fn exit_gracefully() {

    }

    // A new game is started, as opposed to being loaded from file
    fn new() -> Self {
        let game = Self::default();
        let _ = game.save();

        let game_arc = Arc::new(Mutex::new(game));

        let player = Player::init(Arc::clone(&game_arc));
        let _ = player.save();

        let game = Arc::clone(&game_arc);
        let game = *game.lock().unwrap();

        return game;
    }

    // A new game is loaded from file
    fn load() -> Result<Self, &'static str>  {
        let file = load_from_file_bin(Self::GAME_FILEPATH)
            .expect("Failed to load game file");

        // There is no data stored yet
        // Create a new one and return here
        if file.len() <= 0 {
            let game = Self::default();
            let _ = game.save();
            return Ok(game);
        }

        let game: Self = bincode::deserialize(&file)
            .expect("Failed to load game from file");

        Ok(game)
    }

    fn save(&self) -> Result<(), io::Error> {
        let serialized = bincode::serialize(self)
            .expect("Failed to serialize game");

        save_to_file_bin(Self::GAME_FILEPATH, &serialized)
    }
}