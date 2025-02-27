use crossterm::{self, cursor::{position, Hide, MoveTo, Show}, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen}, Command};
use std::io::{stdout,Write,self};

use super::Position;


pub fn queue<T:Command>(command:T) -> io::Result<()>{
    queue!(stdout(),command)?;
    Ok(())
}
pub fn flush()->io::Result<()> {
    stdout().flush()?;
    Ok(())
}
pub fn init()->io::Result<(u16,u16)>{
    enable_raw_mode()?;
    queue(EnterAlternateScreen)?;
    move_cursor(Position(0,0))?;
    hide_cursor()?;
    flush()?;
    Ok(size()?)
}
pub fn deinit()->io::Result<()>{
    queue(LeaveAlternateScreen)?;
    show_cursor()?;
    disable_raw_mode()?;
    flush()?;

    Ok(())
}
pub fn move_cursor(pos:Position)->io::Result<Position>{
    queue(MoveTo(pos.0,pos.1))?;
    Ok(pos)
}

pub fn get_cursor()->io::Result<Position>{
    let temp = position()?;
    Ok(Position(temp.0,temp.1))
}

pub fn show_cursor()->io::Result<()>{
    queue(Show)?;
    Ok(())
}
pub fn hide_cursor()->io::Result<()>{
    queue(Hide)?;
    Ok(())
}

pub fn print(message:&str)->io::Result<()>{
    queue(Print(message))?;
    Ok(())
}
