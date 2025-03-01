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
    fn pieces(&self);
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
        match self{
            TetEnum::Onum(o) => o.clockwise(board),
            TetEnum::Inum(i) => i.clockwise(board),
            TetEnum::Znum(z) => z.clockwise(board),
            TetEnum::Snum(s) => s.clockwise(board),
            TetEnum::Lnum(l) => l.clockwise(board),
            TetEnum::Jnum(j) => j.clockwise(board),
        }
    }

    fn counterwise(&mut self,board:&mut [u8]) {
        match self{
            TetEnum::Onum(o) => o.counterwise(board),
            TetEnum::Inum(i) => i.counterwise(board),
            TetEnum::Znum(z) => z.counterwise(board),
            TetEnum::Snum(s) => s.counterwise(board),
            TetEnum::Lnum(l) => l.counterwise(board),
            TetEnum::Jnum(j) => j.counterwise(board),
        }
    }

    fn up(&mut self,board:&mut [u8]) {
        match self{
            TetEnum::Onum(o) => o.up(board),
            TetEnum::Inum(i) => i.up(board),
            TetEnum::Znum(z) => z.up(board),
            TetEnum::Snum(s) => s.up(board),
            TetEnum::Lnum(l) => l.up(board),
            TetEnum::Jnum(j) => j.up(board),
        }
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
        
    }
}
impl TetEnum{

}
impl Default for TetEnum{
    fn default() -> Self {
        match random_range(0..6){
            0 => TetEnum::Onum(O::default()),
            1 => TetEnum::Inum(I::default()),
            2 => TetEnum::Lnum(L::default()),
            3 => TetEnum::Jnum(J::default()),
            4 => TetEnum::Snum(S::default()),
            5 => TetEnum::Znum(Z::default()),
            _ => unreachable!()
        }
    }
}
