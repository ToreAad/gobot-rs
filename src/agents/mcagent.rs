use super::agent::Agent;
use rand::{thread_rng, Rng};
use crate::gamestate::{GameState, is_point_an_eye, legal_moves};
use crate::gotypes::Move;
use crate::{gotypes::{Point}};



pub struct McAgent{}

impl Agent for McAgent{
    fn select_move(&self, game_states: &Vec<GameState>) -> Move{
        let mut candidates = Vec::new();
        let last_state = &game_states[game_states.len()-1];
        for r in 1..last_state.board.num_rows+1{
            for c in 1..last_state.board.num_cols+1{
                let point = Point{row: r, col: c};
                if GameState::is_valid_move(game_states, Move::Play(point)) && !is_point_an_eye(&last_state.board, point, last_state.next_player){
                    candidates.push(point);
                }
            }
        }
        if candidates.len() == 0{
            return Move::Pass;
        }
        let mut rng = thread_rng();
        Move::Play(candidates[rng.gen_range(0..candidates.len())])
    }
}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use super::*;
    use crate::{gamestate::GameState, gotypes};

}