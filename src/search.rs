use crate::models::Player;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

struct SearchResult {
    player: Player,
    ranking: i64,
}

pub fn search(query: String, players: &Vec<Player>) -> Vec<Player> {
    let matcher = SkimMatcherV2::default();
    let mut output: Vec<SearchResult> = Vec::new();
    for player in players {
        let name_score = {
            let first_name_score = matcher.fuzzy_match(&player.last_name, &query);
            let last_name_score = matcher.fuzzy_match(&player.first_name, &query);
            let full_name_score =
                matcher.fuzzy_match(&(player.first_name.clone() + &player.last_name), &query);
            let mut res = std::cmp::max(first_name_score, last_name_score);
            std::cmp::max(res, full_name_score)
        };

        let team_score = matcher.fuzzy_match(&player.team, &query);
        let position_score = matcher.fuzzy_match(&player.position, &query);

        let mut total_score = 0;
        match name_score {
            Some(name_score) => {
                total_score += name_score * (70 as i64);
            }
            None => {}
        }
        match team_score {
            Some(team_score) => {
                total_score += team_score * (20 as i64);
            }
            None => {}
        }
        match position_score {
            Some(position_score) => {
                total_score += position_score * (10 as i64);
            }
            None => {}
        }
        if total_score > 0 {
            output.push(SearchResult {
                player: player.clone(),
                ranking: total_score,
            });
        }
    }

    output.sort_by(|a, b| b.ranking.cmp(&a.ranking));
    let players_output = output.into_iter().map(|a| a.player).collect::<Vec<_>>();
    if players_output.len() > 0 {
        players_output
    } else {
        players.clone()
    }
}
