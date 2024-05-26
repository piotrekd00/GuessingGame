use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GameConfig {
    pub lower_bound: i32,
    pub upper_bound: i32,
}

#[derive(Deserialize)]
pub struct Guess {
    pub player_name: String,
    pub num: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Score {
    pub name: String,
    pub attempts: u32,
    pub date: u64,
    pub guessed_number: i32,
}

pub struct GameState {
    pub secret_number: i32,
    pub attempts: u32,
    pub started: bool,
    pub last_activity: SystemTime,
    pub scores: Vec<Score>,
}

pub type GameMap = Arc<Mutex<HashMap<Uuid, GameState>>>;


