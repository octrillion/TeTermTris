
use super::{Position, Tetrimino};


pub struct J{
    piece_array:[Position;4]
}
impl Tetrimino for J{
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

    fn pieces(&self) {
        todo!()
    }
}
impl Default for J{
    fn default() -> Self {
        Self{
            piece_array:[Position(0,3),Position(0,4),Position(0,5),
                         Position(1,3)
                        ]
        }
    }
}
