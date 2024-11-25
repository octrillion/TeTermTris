use crossterm::{
    terminal::{EnterAlternateScreen,LeaveAlternateScreen,enable_raw_mode,disable_raw_mode}, 
    queue,Command
};
use std::io::{self, Write,stdout};

pub struct Terminal;

impl Terminal {
    pub fn init() -> io::Result<()>{
        enable_raw_mode()?;
        Self::enter_alternative_screen()?;
        Self::flush()?;
        Ok(())
    }
    pub fn close() -> io::Result<()>{
        Self::exist_alternative_screen()?;
        disable_raw_mode()?;
        Self::flush()?;
        Ok(())
    }

    pub fn flush() -> io::Result<()>{
        stdout().flush()?;
        Ok(())
    }
    pub fn queue<T:Command>(command:T) -> io::Result<()>{
        queue!(stdout(),command)?;
        Ok(())
    }
    fn enter_alternative_screen() -> io::Result<()>{
        Self::queue(EnterAlternateScreen)?;
        Ok(())
    }
    fn exist_alternative_screen() -> io::Result<()>{
        Self::queue(LeaveAlternateScreen)?;
        Ok(())
    }
}
