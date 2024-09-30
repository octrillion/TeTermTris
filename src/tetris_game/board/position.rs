use std::ops::{Add, AddAssign, Sub};

#[derive(Eq,PartialEq,Clone,Copy,Debug)]
pub struct Position{
    pub row:i64,
    pub column:i64,
}
impl Add for Position{
    type Output = Self;
    fn add(self, rhs:Self)-> Self::Output{
        Self{
            row: self.row + rhs.row,
            column: self.column + rhs.column
        }
    }
}
impl AddAssign for Position{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}



#[cfg(test)]
mod test{
    use super::*;

    static POS1:Position = Position{row:1,column:1};
    static POS2:Position = Position{row:1,column:-1};

    #[test]
    fn test_equals(){
        assert!(POS1 == Position{row:1,column:1})
    }
    #[test]
    fn test_partial_equals(){
        assert!(POS1 != POS2);
    }
    #[test]
    fn test_add(){
        assert_eq!(POS1+POS2,Position{row:2,column:0});
    }
    #[test]
    fn test_add_assign(){
        let mut pos3 = Position{row:0,column:0};
        pos3+=POS2;
        assert_eq!(pos3,Position{row:1,column:-1});
    }
}
