mod card;
mod player;
mod game;
mod evaluation_engine;

use game::Game;

fn main() {
    let game = Game::new();
    println!("{:?}", game);
}
