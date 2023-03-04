use std::fmt;


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point{
    pub row: i32,
    pub col: i32,
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Point{

    pub fn new(row: i32, col: i32) -> Point{
        Point{row, col}
    }
    pub fn neighbours(&self) -> Vec<Point>{
        vec![
            Point{row: self.row - 1, col: self.col},
            Point{row: self.row + 1, col: self.col},
            Point{row: self.row, col: self.col - 1},
            Point{row: self.row, col: self.col + 1},
        ]
    }
}

#[cfg(test)]
mod test{
    use super::*;


    #[test]
    fn test_neighbours(){
        let point = Point{row: 2, col: 3};
        let neighbours = point.neighbours();
        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&Point{row: 1, col: 3}));
        assert!(neighbours.contains(&Point{row: 3, col: 3}));
        assert!(neighbours.contains(&Point{row: 2, col: 2}));
        assert!(neighbours.contains(&Point{row: 2, col: 4}));
    }
}