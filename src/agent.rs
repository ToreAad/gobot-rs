use rand::{thread_rng, Rng};
use crate::gamestate::{GameState, is_point_an_eye};
use crate::gotypes::Move;
use crate::{gotypes::{Player, Point}};


pub trait Agent{
    fn select_move(&self, game_states: &Vec<GameState>) -> Move;
}

pub struct RandomAgent{}

impl Agent for RandomAgent{
    fn select_move(&self, game_states: &Vec<GameState>) -> Move{
        let mut candidates = Vec::new();
        let last_state = &game_states[game_states.len()-1];
        for r in 0..last_state.board.num_rows{
            for c in 0..last_state.board.num_cols{
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

fn print_move(player: Player, move_: Move) -> String{
    match move_{
        Move::Play(point) => {
            format!("{} plays at {}", player, point)
        }
        Move::Pass => {
            format!("{} passes", player)
        }
        Move::Resign => {
            format!("{} resigns", player)
        }
    }
}
