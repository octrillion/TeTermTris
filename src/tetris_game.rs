mod board;

use board::Board;

#[derive(Default)]
pub struct Game{
    board:Board,
    score: u64
}
