mod tetrimino;
mod position;

use position::Position; use tetrimino::{Tetrimino, gen_tetrimino};

pub struct Board{
    board:[bool;240],
    curr_piece:Box<dyn Tetrimino>,
}

impl Board{
    const ROW_SIZE:usize = 24;
    const COLUMN_SIZE:usize = 10;

    /// returns true if in bounds
    pub fn check_bounds(&self,positions:Vec<Position>) -> bool{
        for position in positions {
            let Ok(row_us) = usize::try_from(position.row) else{return false};
            let Ok(col_us) = usize::try_from(position.column) else{return false};
            
            println!("({},{})",row_us,col_us);
            if
               row_us < Self::ROW_SIZE  &&
               col_us < Self::COLUMN_SIZE 
            {
                if self.board[row_us * Self::COLUMN_SIZE + col_us]{
                    return false;
                }
            }
            else{
                return false;
            }

        }

        return true;
    }

}
impl Default for Board{
    fn default() -> Self {
        Self{
            board: [false;Self::ROW_SIZE * Self::COLUMN_SIZE],
            curr_piece:gen_tetrimino(),
        }
    }
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn check_bounds_in(){
        let b1:Board = Board::default();
        let positions = vec![
            Position{row:10,column:5},
            Position{row:10,column:6},
            Position{row:11,column:7},
        ];
        assert!(b1.check_bounds(positions));

    }
    #[test]
    fn check_bounds_overlap(){
        let b1:Board = Board{
            curr_piece:gen_tetrimino(),
            board:[true;Board::ROW_SIZE * Board::COLUMN_SIZE],
        };
        let positions = vec![
            Position{row:10,column:5},
        ];
        assert!(!b1.check_bounds(positions));

    }
    #[test]
    fn check_bounds_out(){
        let b1:Board = Board::default();

        let p1= vec![Position{row:0,column:25}];
        assert!(!(b1.check_bounds(p1)));

        let p2= vec![Position{row:100,column:5}];
        assert!(!(b1.check_bounds(p2)));
    }
    #[test]
    fn check_bounds_out_neg(){
        let b1:Board = Board::default();

        let p3= vec![Position{row:-50,column:50}];
        assert!(!(b1.check_bounds(p3)));

        let p4= vec![Position{row:50,column:-50}];
        assert!(!(b1.check_bounds(p4)));
    }

}
