use std::fmt;

use super::point::Point;

#[derive(Clone, Copy)]
pub enum Action{
    Play(Point),
    Pass,
    Resign
}

impl fmt::Display for Action{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Action::Play(point) => write!(f, "Play at {}", point),
            Action::Pass => write!(f, "Pass"),
            Action::Resign => write!(f, "Resign"),
        }
    }
}

