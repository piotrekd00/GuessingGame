use std::time::{Duration, SystemTime};
use tokio::time::interval;
use crate::models::GameMap;

pub async fn cleanup_old_games(game_map: GameMap) {
    let mut interval = interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        let mut games = game_map.lock().unwrap();
        let now = SystemTime::now();
        games.retain(|_, game| {
            now.duration_since(game.last_activity)
                .map(|d| d < Duration::new(30, 0))
                .unwrap_or(false)
        });
    }
}
