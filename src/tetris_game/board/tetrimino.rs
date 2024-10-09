
mod pieces;

use super::position::Position;
use super::{MAX_ROW,MAX_COLUMN};
use pieces::{O,L,J,Z,S,I};

pub enum Direction{
    Up,
    Down,
    Left,
    Right
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

fn turn(direction:Direction, orientation:Orientation)-> Option<Orientation>{
    match direction{
        Direction::Right => { match orientation{
            Orientation::North => Some(Orientation::East),
            Orientation::East => Some(Orientation::South),
            Orientation::South => Some(Orientation::West),
            Orientation::West => Some(Orientation::North),
        }},
        Direction::Left => { match orientation{
            Orientation::North => Some(Orientation::West),
            Orientation::East => Some(Orientation::North),
            Orientation::South => Some(Orientation::East),
            Orientation::West => Some(Orientation::South)
        }},
        _ => None

    }
}
