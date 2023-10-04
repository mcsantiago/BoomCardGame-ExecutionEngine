extern crate lru; 

use lru::LruCache;
use crate::game::Game;

pub struct EvaluationEngine {
    game_cache: LruCache<u32, Game>
}

impl EvaluationEngine {
    pub fn apply_move(&mut self, game_id: u32, move_str: &str) -> Result<(), String> {
        let gameOption = self.game_cache.get(&game_id);

        match gameOption {
            Some(game) => {
                
                Ok(println!("{:?}", game))
            },
            None => Err(format!("Can't find game with id {:?}", game_id))
        }
    }

    pub fn validate_move(self, move_str: &str) -> Result<(), String> {
        // move strings are serialized like So
        // p(id),m(id),(a/d) - Player (id) plays monster (id) in (attack / defense) mode
        todo!()
    }
}

