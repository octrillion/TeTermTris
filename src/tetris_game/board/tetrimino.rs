
pub(super) mod pieces;

use super::position::Position;
use pieces::{O,L};

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}
pub enum Rotation{
    Clockwise,
    CounterClockWise,
}
pub enum Orientation{
    North,
    West,
    East,
    South
}


pub trait Tetrimino{
    fn rotate(&mut self,direction:Direction) -> Vec<Position>;
    fn shift(&mut self,direction:Direction) -> Vec<Position>;
    fn get_positions(&self) -> Vec<Position>;
}




pub fn gen_tetrimino<>() -> Box<dyn Tetrimino>{
    Box::new(O::default())
}

fn turn(rotation:Rotation, orientation:Orientation)-> Orientation{
    match rotation{
        Rotation::Clockwise =>{ match orientation{
            Orientation::North => Orientation::East,
            Orientation::East =>  Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West =>  Orientation::North,
        }},
        Rotation::CounterClockWise=> { match orientation{
            Orientation::North => Orientation::West,
            Orientation::East =>  Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West =>  Orientation::South
        }},

    }
}
