mod card;
mod player;
mod game;
mod board;
mod evaluation_engine;

use game::Game;
use board::Board;

fn main() {
    let game = Game::new();
    println!("{:?}", game);
}
