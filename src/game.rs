mod tetrimino;

use tetrimino::TetEnum;
pub struct Game{
    pub board:[u8;240],
    pub points:u64,
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
