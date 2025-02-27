mod tetris;
mod game;
mod view;
use tetris::Tetris;



fn main(){
#![warn(clippy::all, clippy::pedantic)]
    Tetris::init().unwrap().run();
}
