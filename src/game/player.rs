use std::fmt;

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
