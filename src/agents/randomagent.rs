use super::agent::Agent;
use rand::{thread_rng, Rng};
use crate::gamestate::{GameState, is_point_an_eye};
use crate::gotypes::Move;
use crate::{gotypes::{Point}};


fn legal_moves(game_states: &Vec<GameState>) -> Vec<Point>{
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
    candidates
}

pub struct RandomAgent{}

impl Agent for RandomAgent{
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

    #[test]
    fn test_random_agent()
    {
        let board_size = 2;

        let mut game = GameState::new(board_size);
        let bots = HashMap::from([
            (gotypes::Player::Black, RandomAgent{}),
            (gotypes::Player::White, RandomAgent{}),
        ]);
    
        loop {
            print!("{}[2J", 27 as char);
            let current_game = game.last().unwrap();
            print!("{}", current_game);
            if GameState::is_over(&game) {
                break;
            }
    
            let player = current_game.next_player();
            let agent = bots.get(&player).unwrap();
            let move_ = agent.select_move(&game);
            GameState::apply_move(&mut game, move_);
        }
    }
}