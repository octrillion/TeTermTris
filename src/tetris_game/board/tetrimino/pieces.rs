use super::Tetrimino;
use super::Position;
use super::Direction;
use super::Orientation;



pub struct O {
    pivot_pos:Position,
    orientation:Orientation,
    pos_table: [Position;16],
}
impl Default for O{
    fn default()-> Self{
        Self{
            orientation:Orientation::North,
            pivot_pos:Position{row:0,column:4},
            pos_table:todo!()
        
        }
    }
}


impl Tetrimino for O{
    fn rotate(&mut self,_direction:Direction) -> Vec<Position>{
        todo!()
    }
    fn shift(&mut self,direction:Direction) -> Vec<Position>{
        todo!()
    }

    fn get_positions(&self) -> Vec<Position> {
        todo!()
    }
}

pub struct I;
pub struct L;
pub struct J;
pub struct Z;
pub struct S;
