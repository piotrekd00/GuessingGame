#[macro_use] extern crate rocket;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use rocket_dyn_templates::Template;

mod routes;
mod models;
mod utils;

use routes::{index, files, start, guess, game_page, game_scores};
use models::GameState;
use utils::cleanup_old_games;

#[tokio::main]
async fn main() {
    let game_map = Arc::new(Mutex::new(HashMap::<Uuid, GameState>::new()));
    let game_map_clone = game_map.clone();

    tokio::spawn(async move {
        cleanup_old_games(game_map_clone).await;
    });

    rocket::build()
        .manage(game_map)
        .mount("/", routes![index, files, start, guess, game_page, game_scores])
        .attach(Template::fairing())
        .launch()
        .await
        .expect("server failed to start");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::asynchronous::Client;
    use rocket::http::{Status, ContentType};
    use rocket::serde::json::json;

    #[rocket::async_test]
    async fn test_start_game() {
        let client = Client::tracked(rocket::build()
            .manage(Arc::new(Mutex::new(HashMap::<Uuid, GameState>::new())))
            .mount("/", routes![start])).await.unwrap();

        let response = client.post("/start")
            .header(ContentType::JSON)
            .body(json!({
                "lower_bound": 1,
                "upper_bound": 100
            }).to_string())
            .dispatch().await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_make_guess() {
        let client = Client::tracked(rocket::build()
            .manage(Arc::new(Mutex::new(HashMap::<Uuid, GameState>::new())))
            .mount("/", routes![start, guess])).await.unwrap();

        let response = client.post("/start")
            .header(ContentType::JSON)
            .body(json!({
                "lower_bound": 1,
                "upper_bound": 100
            }).to_string())
            .dispatch().await;

        let game_id: Uuid = response.into_json::<Uuid>().await.unwrap();

        let guess_response = client.post(format!("/guess/{}", game_id))
            .header(ContentType::JSON)
            .body(json!({
                "player_name": "Player1",
                "num": 50
            }).to_string())
            .dispatch().await;

        assert_eq!(guess_response.status(), Status::Ok);
        let body = guess_response.into_string().await.unwrap();
        assert!(body.contains("Too low") || body.contains("Too high") || body.contains("Congratulations"));
    }

    #[rocket::async_test]
    async fn test_game_scores() {
        let client = Client::tracked(rocket::build()
            .manage(Arc::new(Mutex::new(HashMap::<Uuid, GameState>::new())))
            .mount("/", routes![start, game_scores])).await.unwrap();

        let response = client.post("/start")
            .header(ContentType::JSON)
            .body(json!({
                "lower_bound": 1,
                "upper_bound": 100
            }).to_string())
            .dispatch().await;

        let game_id: Uuid = response.into_json::<Uuid>().await.unwrap();

        let scores_response = client.get(format!("/game/{}/scores", game_id))
            .dispatch().await;

        assert_eq!(scores_response.status(), Status::Ok);
        let scores: Vec<models::Score> = scores_response.into_json().await.unwrap();
        assert!(scores.is_empty());
    }

}