use crate::game::game::Game;
use crate::game::action::Action;

pub trait Agent{
    fn select_action(&self, game: &dyn Game) -> Action;
}