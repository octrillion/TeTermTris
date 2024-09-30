
use super::position::Position;
use super::{MAX_ROW,MAX_COLUMN};

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

pub struct O {
    block_positions:[Position;4],
    orientation:Orientation,
}
impl Default for O{
    fn default()-> Self{
        Self{
            block_positions:[
                Position{row:0,column:0},
                Position{row:0,column:1},
                Position{row:1,column:0},
                Position{row:1,column:1},
            ],
            orientation:Orientation::North,
        }
    }
}
impl Tetrimino for O{
    fn rotate(&mut self,direction:Direction) -> Vec<Position>{
        Vec::from(self.block_positions)
    }
    fn shift(&mut self,direction:Direction) -> Vec<Position>{
        let offset:Position = match direction {
            Direction::Left => Position{row:-1,column:0},
            Direction::Right => Position{row:1,column:0},
            Direction::Up => Position{row:0,column:-1},
            Direction::Down => Position{row:0,column:1},
        };

        for i in 0..4{
            self.block_positions[i] += offset;
        }
        Vec::from(self.block_positions)
    }
    fn get_positions(&self) -> Vec<Position>{
        Vec::from(self.block_positions)
    }
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
