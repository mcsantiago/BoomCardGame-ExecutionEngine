extern crate lru; 

use lru::LruCache;
use crate::game::Game;
use crate::card::Card;
use std::sync::Arc;

pub struct EvaluationEngine {
    game_cache: LruCache<u32, Game>,
    reference_deck: Arc<Card>
}

impl EvaluationEngine {
    pub fn apply_move(&mut self, game_id: u32, move_str: &str) -> Result<(), String> {
        let game_option = self.game_cache.get(&game_id);

        match game_option {
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

