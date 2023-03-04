use std::fmt;

use crate::game::{player::Player, game::Game, action::Action, score::Score};

use super::{gamestate::GameState, scoring::GameResult};


pub struct Go {
    states: Vec<GameState>,
    current_player: Player,
}

impl Go{
    pub fn new(board_size: i32) -> Go{
        let states = GameState::new(board_size);
        let current_player = states.last().unwrap().next_player;
        Go{states: states, current_player:current_player}
    }   
}

impl Game for Go{
    fn legal_moves(&self) -> Vec<Action>{
        GameState::legal_moves(&self.states)
    }

    fn is_over(&self) -> bool{
        GameState::is_over(&self.states)
    }

    fn score(&self) -> Box<dyn Score>{
        let current_game = self.states.last().unwrap();
        let result = GameResult::new(&current_game.board, 0.0);
        Box::new(result)
    }

    fn apply_move(&mut self, action: Action) -> bool{
        GameState::apply_move(&mut self.states, action)
    }

    fn current_player(&self) -> Player{
        self.current_player
    }

    fn next_player(&self) -> Player{
        self.current_player.other()
    }
}

impl fmt::Display for Go{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.states[self.states.len()-1])
    }
}

impl Clone for Go{
    fn clone(&self) -> Self{
        Go{states: self.states.clone(), current_player: self.current_player}
    }
}