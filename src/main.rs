#![warn(
    clippy::all,
    clippy::pedantic)]


mod tetris_game;

use tetris_game::Game;
fn main() {
    let game = Game::default();
}
