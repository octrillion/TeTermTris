use crate::{view::{Window,Position},game::Game};
use std::io;

pub struct Tetris{
    game:Game,
    window:Window
}
impl Tetris{
    pub fn init()->io::Result<Self>{
        Ok(Self{
            game:Game::init(),
            window:Window::init()?
        })
    }
    pub fn run(&self){
        let done = false;
        while !done {
            

            self.render();
        }
    }
    fn render(&self){
        let pos = Position(0,0);
        self.window.move_cursor(pos).unwrap();
        self.window.print(&"hello World!").unwrap();
    }
}
