

use super::{Position, Tetrimino};


pub struct I{
    piece_array:[Position;4]
}
impl Tetrimino for I{
    fn clockwise(&mut self,board:&mut [u8]) {
        todo!()
    }

    fn counterwise(&mut self,board:&mut [u8]) {
        todo!()
    }

    fn up(&mut self,board:&mut [u8]) {
        todo!()
    }

    fn down(&mut self,board: &mut [u8]) {
        todo!()
    }

    fn left(&mut self,board: &mut [u8]) {
        todo!()
    }

    fn right(&mut self,board: &mut [u8]) {
        todo!()
    }

    fn pieces(&self, board: &mut [u8]) {
        todo!()
    }
}
impl Default for I{
    fn default() -> Self {
        Self{
            piece_array:
                [Position(0,5),Position(1,5),Position(2,5),Position(3,5)]
        }
    }
}
