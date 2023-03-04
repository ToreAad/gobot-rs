// use super::agent::Agent;
// use rand::{thread_rng, Rng};
// use crate::gamestate::{GameState};
// use crate::game::action::{Action, self};
// use crate::game::player::Player;
// use crate::go::scoring::GameResult;
// use crate::game::point::Point;


// struct McNode{
//     state: Vec<GameState>,
//     parent: Option<Box<McNode>>,
//     children: Vec<Box<McNode>>,
//     visits: i32,
//     wins: i32,
// }

// impl McNode{
//     fn new(state: Vec<GameState>, parent: Option<Box<McNode>>) -> McNode{
//         McNode{
//             state,
//             parent,
//             children: Vec::new(),
//             visits: 0,
//             wins: 0,
//         }
//     }
//     fn add_child(&mut self, child: Box<McNode>){
//         self.children.push(child);
//     }
//     fn update(&mut self, result: i32){
//         self.visits += 1;
//         self.wins += result;
//     }
//     fn uct(&self) -> f32{
//         let parent_visits = match &self.parent{
//             Some(parent) => parent.visits,
//             None => 1,
//         };
//         let win_rate = self.wins as f32 / self.visits as f32;
//         let exploration = (2.0 * (parent_visits as f32).ln() / self.visits as f32).sqrt();
//         win_rate + exploration
//     }
//     fn best_child(&self) -> Option<&Box<McNode>>{
//         self.children.iter().max_by(|a, b| a.uct().partial_cmp(&b.uct()).unwrap())
//     }
//     fn is_terminal(&self) -> bool{
//         self.children.len() == 0
//     }
//     fn rollout(&self) -> f32{
//         let mut game = self.state.clone();
//         let mut rng = thread_rng();
//         loop{
//             let moves = GameState::legal_moves(&game);
//             if moves.len() == 0{
//                 break;
//             }
//             let move_ = moves[rng.gen_range(0..moves.len())].clone();
//             GameState::apply_move(&mut game, move_);
//         }
//         let current_state = game.last().unwrap();
//         let board = &current_state.board;
//         let score = GameResult::new(board, 7.5);
//         match current_state.next_player{
//             Player::Black => score.black_score,
//             Player::White => score.white_score,
//         }
//     }
//     fn backpropagate(&mut self, result: i32){
//         self.update(result);
//         match &mut self.parent{
//             Some(parent) => parent.backpropagate(result),
//             None => (),
//         }
//     }
// }

// pub struct McAgent{}

// impl Agent for McAgent{
//     fn select_move(&self, game_states: &Vec<GameState>) -> Action{
//         let mut candidates = Vec::new();
//         let last_state = &game_states[game_states.len()-1];
//         for r in 1..last_state.board.num_rows+1{
//             for c in 1..last_state.board.num_cols+1{
//                 let point = Point{row: r, col: c};
//                 if GameState::is_valid_move(game_states, Action::Play(point)) && !last_state.board.is_point_an_eye( point, last_state.next_player){
//                     candidates.push(point);
//                 }
//             }
//         }
//         if candidates.len() == 0{
//             return Action::Pass;
//         }
//         let mut rng = thread_rng();
//         Action::Play(candidates[rng.gen_range(0..candidates.len())])
//     }
// }

// #[cfg(test)]
// mod tests{
//     use std::collections::HashMap;

//     use super::*;
//     use crate::{gamestate::GameState};

// }