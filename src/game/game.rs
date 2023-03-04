use super::{action::Action, score::Score, player::Player};

pub trait Game : std::fmt::Display {
    fn legal_moves(&self) -> Vec<Action>;
    fn is_over(&self) -> bool;
    fn score(&self) -> Box<dyn Score>;
    fn apply_move(&mut self, action: Action) -> bool;
    fn current_player(&self) -> Player;
    fn next_player(&self) -> Player;
}