use crate::gamestate::{GameState};
use crate::gotypes::Move;

pub trait Agent{
    fn select_move(&self, game_states: &Vec<GameState>) -> Move;
}