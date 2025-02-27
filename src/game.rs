mod tetrimino;
mod position;

use tetrimino::TetEnum;
pub struct Game{
    board:[u8;240],
    points:u64,
    bag:Vec<TetEnum>
}


impl Game{
    pub fn init()-> Self{
        Self{
            board:[0;240],
            points:0,
            bag:Vec::new()
        }
    }
}
