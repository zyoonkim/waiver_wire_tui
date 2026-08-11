use std::collections::HashSet;

use crate::models::Player;

#[derive(Clone)]
pub struct ApiClient {
    wishlist: HashSet<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            wishlist: HashSet::from(["1".to_string(), "4".to_string()]),
        }
    }

    pub async fn get_players(&self) -> Result<Vec<Player>, ()> {
        Ok(vec![
            Player {
                player_id: "1".to_string(),
                first_name: "Puka".to_string(),
                last_name: "Nacua".to_string(),
                position: "WR".to_string(),
                team: "LA Rams".to_string(),
                number: 17,
            },
            Player {
                player_id: "2".to_string(),
                first_name: "Justin".to_string(),
                last_name: "Jefferson".to_string(),
                position: "WR".to_string(),
                team: "Minnesota Vikings".to_string(),
                number: 18,
            },
            Player {
                player_id: "3".to_string(),
                first_name: "Ja'Marr".to_string(),
                last_name: "Chase".to_string(),
                position: "WR".to_string(),
                team: "Cincinnati Bengals".to_string(),
                number: 1,
            },
            Player {
                player_id: "4".to_string(),
                first_name: "Breece".to_string(),
                last_name: "Hall".to_string(),
                position: "RB".to_string(),
                team: "New York Jets".to_string(),
                number: 20,
            },
            Player {
                player_id: "5".to_string(),
                first_name: "Marvin".to_string(),
                last_name: "Harrison".to_string(),
                position: "WR".to_string(),
                team: "Arizona Cardinals".to_string(),
                number: 18,
            },
            Player {
                player_id: "6".to_string(),
                first_name: "Malik".to_string(),
                last_name: "Nabers".to_string(),
                position: "WR".to_string(),
                team: "New York Giants".to_string(),
                number: 1,
            },
            Player {
                player_id: "7".to_string(),
                first_name: "CeeDee".to_string(),
                last_name: "Lamb".to_string(),
                position: "WR".to_string(),
                team: "Dallas Cowboys".to_string(),
                number: 88,
            },
        ])
    }

    pub async fn get_wishlist_ids(&self) -> Result<Vec<String>, ()> {
        Ok(self.wishlist.iter().cloned().collect())
    }

    pub async fn get_wishlist(&self, players: &[Player]) -> Result<Vec<Player>, ()> {
        let player_ids: HashSet<String> = self.get_wishlist_ids().await?.into_iter().collect();

        Ok(players
            .iter()
            .filter(|player| player_ids.contains(&player.player_id))
            .cloned()
            .collect())
    }

    pub async fn add_to_wishlist(&mut self, player_id: &str) -> Result<(), ()> {
        self.wishlist.insert(player_id.to_string());
        Ok(())
    }

    pub async fn remove_from_wishlist(&mut self, player_id: &str) -> Result<(), ()> {
        self.wishlist.remove(player_id);
        Ok(())
    }
}
