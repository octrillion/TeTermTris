use crate::view::View;
pub struct Position{
    row:usize,
    col:usize
}

pub struct Object{
    block_pos:[Position;4]
}
    

pub struct Game{
    score:u16,
    objects:Vec<Object>,
    view:View
}

