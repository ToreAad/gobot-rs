use std::{collections::{HashSet, HashMap}, fmt};

use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Player{
    Black,
    White,
}

impl fmt::Display for Player{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Black => write!(f, "Black"),
            Player::White => write!(f, "White"),
        }
    }
}

impl Player{
    pub fn other(&self) -> Player{
        match self{
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

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
    pub fn neighbours(&self) -> Vec<Point>{
        vec![
            Point{row: self.row - 1, col: self.col},
            Point{row: self.row + 1, col: self.col},
            Point{row: self.row, col: self.col - 1},
            Point{row: self.row, col: self.col + 1},
        ]
    }
}


#[derive(Clone)]
pub enum Move{
    Play(Point),
    Pass,
    Resign
}

impl fmt::Display for Move{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Move::Play(point) => write!(f, "Play at {}", point),
            Move::Pass => write!(f, "Pass"),
            Move::Resign => write!(f, "Resign"),
        }
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