
mod o;
mod i;
mod j;
mod l;
mod z;
mod s;

use i::I;
use o::O;
use z::Z;
use j::J;
use l::L;
use s::S;
use rand::random_range;
pub struct Position(usize,usize);


pub trait Tetrimino:Default{
    fn clockwise(&mut self,board:&mut [u8]);
    fn counterwise(&mut self,board:&mut [u8]);
    fn up(&mut self,board:&mut [u8]);
    fn down(&mut self,board: &mut [u8]);
    fn left(&mut self,board: &mut [u8]);
    fn right(&mut self,board: &mut [u8]);
    fn pieces(&self, board: &mut [u8]);
}

pub enum TetEnum{
    Onum(O),
    Inum(I),
    Znum(Z),
    Snum(S),
    Lnum(L),
    Jnum(J)
}
impl Tetrimino for TetEnum{
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
impl TetEnum{
    fn place(&self,board: &mut [u8]){
        todo!()
    }

}
impl Default for TetEnum{
    fn default() -> Self {
        let tetnum = match random_range(0..6){
            0 => TetEnum::Onum(O::default()),
            1 => TetEnum::Inum(I::default()),
            2 => TetEnum::Lnum(L::default()),
            3 => TetEnum::Jnum(J::default()),
            4 => TetEnum::Snum(S::default()),
            5 => TetEnum::Znum(Z::default()),
            _ => unreachable!()
        };
    }
}
