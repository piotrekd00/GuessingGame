use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;
use rand::Rng;

use crate::models::{GameConfig, Guess, Score, GameState, GameMap};

#[post("/start", data = "<game>")]
pub fn start(game: Json<GameConfig>, state: &State<GameMap>) -> Json<Uuid> {
    let secret_number = rand::thread_rng().gen_range(game.lower_bound..=game.upper_bound);
    let game_state = GameState {
        secret_number,
        attempts: 0,
        started: true,
        scores: Vec::new(),
        last_activity: std::time::SystemTime::now(),
    };
    let mut games = state.lock().unwrap();
    let game_id = Uuid::new_v4();
    games.insert(game_id, game_state);
    Json(game_id)
}

#[post("/guess/<game_id>", data = "<guess>")]
pub fn guess(game_id: &str, guess: Json<Guess>, state: &State<GameMap>) -> String {
    let game_id = match Uuid::parse_str(&game_id) {
        Ok(id) => id,
        Err(_) => return "Invalid game ID".to_string(),
    };

    let mut games = state.lock().unwrap();
    let game_state = match games.get_mut(&game_id) {
        Some(state) => state,
        None => return "Game not found".to_string(),
    };

    if !game_state.started {
        return "Game not started".to_string();
    }

    game_state.attempts += 1;
    game_state.last_activity = std::time::SystemTime::now();
    match guess.num.cmp(&game_state.secret_number) {
        std::cmp::Ordering::Less => "Too low".to_string(),
        std::cmp::Ordering::Greater => "Too high".to_string(),
        std::cmp::Ordering::Equal => {
            let score = Score {
                name: guess.player_name.clone(),
                attempts: game_state.attempts,
                date: std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                guessed_number: game_state.secret_number,
            };
            game_state.scores.push(score);
            game_state.started = false;
            if game_state.scores.iter().all(|s| s.attempts >= game_state.attempts) {
                "Congratulations, you guessed the number! You have the best score!".to_string()
            } else {
                "Congratulations, you guessed the number!".to_string()
            }
        }
    }
}

#[get("/game/<game_id>/scores")]
pub fn game_scores(game_id: &str, state: &State<GameMap>) -> Json<Vec<Score>> {
    let game_id = match Uuid::parse_str(&game_id) {
        Ok(id) => id,
        Err(_) => return Json(vec![]),
    };

    let games = state.lock().unwrap();
    if let Some(game_state) = games.get(&game_id) {
        Json(game_state.scores.clone())
    } else {
        Json(vec![])
    }
}

#[get("/")]
pub fn index(state: &State<GameMap>) -> Template {
    let games = state.lock().unwrap();
    let game_ids: Vec<String> = games.keys().map(|id| id.to_string()).collect();
    Template::render("index", context! { games: game_ids })
}

#[get("/game/<game_id>")]
pub fn game_page(game_id: &str) -> Template {
    Template::render("game", &game_id)
}

#[get("/static/<file>")]
pub async fn files(file: &str) -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(std::path::Path::new("static/").join(file)).await.ok()
}
