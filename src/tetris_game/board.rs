mod tetrimino;
mod position;
use tetrimino::{Tetrimino, gen_tetrimino};

const MAX_ROW:usize = 10;
const MAX_COLUMN:usize = 24;
pub struct Board{
    board:[bool;240],
    curr_piece:Box<dyn Tetrimino>,
}

impl Board{
}
impl Default for Board{
    fn default() -> Self {
        Self{
            board: [false;240],
            curr_piece:gen_tetrimino(),
        }
    }
}
