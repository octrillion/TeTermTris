use super::{Position, Tetrimino};


pub struct O{
    piece_array:[Position;4]
}


impl Tetrimino for O {
    fn clockwise(&mut self,board:&mut [u8]){
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
impl Default for O{
    fn default() -> Self {
        
    return Self{
        piece_array:[Position(4,0),Position(5,0),
                     Position(4,1),Position(5,1)]
        }
    }
}
