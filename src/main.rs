mod view;
mod game;

use std::{thread::sleep, time::Duration};

use crossterm::style::Print;
use view::terminal::Terminal;
fn main(){
    Terminal::init().unwrap();
    Terminal::queue(Print("Hello World!")).unwrap();
    Terminal::flush().unwrap();
    sleep(Duration::from_secs(2));
    Terminal::close().unwrap();
}
