use super::position::Position;
pub enum Direction{
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub enum Orientation{
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
impl Orientation{}


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
            orientation:Orientation::NORTH,
        }
    }
}
impl Tetrimino for O{
    fn rotate(&mut self,direction:Direction) -> Vec<Position>{
        Vec::from(self.block_positions)
    }
    fn shift(&mut self,direction:Direction) -> Vec<Position>{
        let offset:Position = match direction {
            Direction::LEFT => Position{row:-1,column:0},
            Direction::RIGHT => Position{row:1,column:0},
            Direction::UP => Position{row:0,column:-1},
            Direction::DOWN => Position{row:0,column:1},
        };

        for i in 0..4{
            self.block_positions[i] += offset;
        }
        return Vec::from(self.block_positions);
    }
    fn get_positions(&self) -> Vec<Position>{
        Vec::from(self.block_positions)
    }
}




pub fn gen_tetrimino<>() -> Box<dyn Tetrimino>{
    Box::new(O::default())
}

