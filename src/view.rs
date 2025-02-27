mod terminal;

use std::{io, panic::{set_hook, take_hook}};
pub struct Window{
    height:u16,
    width:u16,
    cursor:Position
}
impl Window{
    pub fn init() -> io::Result<Self>{
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = terminal::deinit();
            current_hook(panic_info);
        }));
 
        let (height,width) = terminal::init()?;
        Ok(Self{
            height,
            width,
            cursor:Position(0,0)
        })
    }
    pub fn move_cursor(&self,pos:Position)->io::Result<()>{
        terminal::move_cursor(pos)?;
        Ok(())
    }
    pub fn flush(&self)->io::Result<()>{
        terminal::flush()?;
        Ok(())
    }

    pub fn print(&self, message:&str)->io::Result<()>{
        terminal::print(message)?;
        Ok(())
    }
}

pub struct Position(pub u16,pub u16);
