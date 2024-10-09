use super::Tetrimino;
use super::Position;
use super::Direction;
use super::Orientation;
use super::turn;



pub struct O {
    block_positions:[Position;4],
    orientation:Orientation,
}
impl Default for O{
    fn default()-> Self{
        Self{
            block_positions:[
                Position{row:0,column:4},
                Position{row:0,column:4},
                Position{row:1,column:5},
                Position{row:1,column:5},
            ],
            orientation:Orientation::North,
        }
    }
}


impl Tetrimino for O{
    fn rotate(&mut self,_direction:Direction) -> Vec<Position>{
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

pub struct I{
    block_orientation:[Position]
}
