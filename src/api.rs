use std::collections::HashSet;

use reqwest::Client;

use crate::models::Player;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://127.0.0.1:3000".to_string(),
        }
    }

    pub async fn get_players(&self) -> Result<Vec<Player>, reqwest::Error> {
        self.client
            .get(format!("{}/players", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Player>>()
            .await
    }

    pub async fn get_watchlist_ids(&self) -> Result<Vec<String>, reqwest::Error> {
        self.client
            .get(format!("{}/wishlist", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<String>>()
            .await
    }

    pub async fn get_watchlist(
        &self,
        players: &[Player],
    ) -> Result<Vec<Player>, reqwest::Error> {
        let player_ids: HashSet<String> =
            self.get_watchlist_ids().await?.into_iter().collect();

        Ok(players
            .iter()
            .filter(|player| player_ids.contains(&player.player_id))
            .cloned()
            .collect())
    }

    pub async fn add_to_watchlist(
        &self,
        player_id: &str,
    ) -> Result<(), reqwest::Error> {
        self.client
            .post(format!("{}/wishlist/{}", self.base_url, player_id))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn remove_from_watchlist(
        &self,
        player_id: &str,
    ) -> Result<(), reqwest::Error> {
        self.client
            .delete(format!("{}/wishlist/{}", self.base_url, player_id))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}