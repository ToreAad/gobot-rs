use std::fmt;
use crate::game::{player::Player, action::Action, point::Point};

use super::{goboard::Board};

#[derive(Clone)]
pub struct GameState{
    pub board: Board,
    pub next_player: Player,
    pub last_move: Option<Action>,
}

// pub fn legal_moves(game_states: &Vec<GameState>) -> Vec<Action>{
//     let mut candidates = Vec::new();
//     let last_state = &game_states[game_states.len()-1];
//     for r in 1..last_state.board.num_rows+1{
//         for c in 1..last_state.board.num_cols+1{
//             let point = Point{row: r, col: c};
//             if GameState::is_valid_move(game_states, Action::Play(point)) && !is_point_an_eye(&last_state.board, point, last_state.next_player){
//                 candidates.push(Action::Play(point));
//             }
//         }
//     }
//     candidates
// }

impl fmt::Display for GameState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to play\n{}", self.next_player, self.board)
    }
}


impl GameState{
    pub fn is_over(states: &Vec<GameState>) -> bool{
        let last_state = &states[states.len()-1];

        match last_state.last_move{
            None => false,
            Some(Action::Play(_)) => false,
            Some(Action::Resign) => true,
            Some(Action::Pass) => {
                match states.get(states.len()-2){
                    None => false,
                    Some(state) => match state.last_move{
                        None => false,
                        Some(Action::Pass) => true,
                        _ => false,
                    }
                }
            }
        }
    }
    pub fn situation(&self) -> (Player, u64){
        (self.next_player, self.board.hash())
    }

    pub fn does_move_violate_ko(states: &Vec<GameState>, move_: Action) -> bool{
        let point = match move_{
            Action::Play(point) => {
                point
            }
            _ => return false,
        };

        let last_state = match states.last(){
            None => return false,
            Some(state) => state,
        };

        let mut next_board = last_state.board.clone();
        next_board.place_stone(last_state.next_player, point);
        let next_situation = (last_state.next_player.other(), next_board.hash());
        for state in states.iter().rev(){
            if state.situation() == next_situation{
                return true;
            }
        }
        false

    }

    pub fn is_valid_move(states: &Vec<GameState>, move_: Action) -> bool{
        let last_state = &states[states.len()-1];
        let board = &last_state.board;
        let player = last_state.next_player;
        if GameState::is_over(states){
            return false;
        }
        match move_{
            Action::Pass => {
                return true;
            }
            Action::Resign => {
                return true;
            }
            Action::Play(point) => {
                if !board.is_on_grid(&point){
                    return false;
                }
                if board.get(&point).is_some(){
                    return false;
                }
                if GameState::does_move_violate_ko(states, move_){
                    return false;
                }
                if GameState::is_move_self_capture(&board, player, point){
                    return false;
                }
            }
        }
        true
    }

    pub fn is_move_self_capture(board: &Board, player: Player, point: Point) -> bool{
        board.is_self_capture(player, &point)
    }

    pub fn apply_move(states: &mut Vec<GameState>, move_: Action) -> bool{
        if !GameState::is_valid_move(states, move_.clone()){
            return false;
        }

        let prev_state = states.get(states.len()-1).unwrap();

        let mut next_state = GameState{
            board: prev_state.board.clone(),
            next_player: prev_state.next_player.other(),
            last_move: Some(move_.clone()),
        };
        match move_{
            Action::Play(point) => {
                next_state.board.place_stone(prev_state.next_player, point);
            }
            Action::Pass => {
                next_state.next_player = prev_state.next_player.other();
            }
            Action::Resign => {
                next_state.next_player = prev_state.next_player.other();
            }
        }
        states.push(next_state);
        true
    }

    pub fn new(board_size: i32) -> Vec<GameState>{
        vec![GameState{
            board: Board::new(board_size),
            next_player: Player::Black,
            last_move: None,
        }]
    }

    pub fn legal_moves(game_states: &Vec<GameState>) -> Vec<Action>{
        let mut candidates = Vec::new();
        let last_state = &game_states[game_states.len()-1];
        for r in 1..last_state.board.num_rows+1{
            for c in 1..last_state.board.num_cols+1{
                let point = Point{row: r, col: c};
                if GameState::is_valid_move(game_states, Action::Play(point)) && !last_state.board.is_point_an_eye(point, last_state.next_player){
                    candidates.push(Action::Play(point));
                }
            }
        }
        candidates
    }
}




#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_does_move_violate_ko(){
        let mut states = GameState::new(5);
        GameState::apply_move(&mut states, Action::Play(Point{row: 1, col: 3}));
        GameState::apply_move(&mut states, Action::Play(Point{row: 3, col: 2}));
        GameState::apply_move(&mut states, Action::Play(Point{row: 2, col: 2}));
        GameState::apply_move(&mut states, Action::Play(Point{row: 3, col: 4}));
        GameState::apply_move(&mut states, Action::Play(Point{row: 2, col: 4}));
        let mut violates_ko = GameState::does_move_violate_ko(&states, Action::Play(Point{row: 4, col: 3}));
        assert_eq!(violates_ko, false);
        GameState::apply_move(&mut states, Action::Play(Point{row: 4, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Action::Play(Point{row: 3, col: 3}));
        assert_eq!(violates_ko, false);
        GameState::apply_move(&mut states, Action::Play(Point{row: 3, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Action::Play(Point{row: 2, col: 3}));
        assert_eq!(violates_ko, false);
        GameState::apply_move(&mut states, Action::Play(Point{row: 2, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Action::Play(Point{row: 3, col: 3}));
        assert_eq!(violates_ko, true);
        violates_ko = GameState::does_move_violate_ko(&states, Action::Play(Point{row: 3, col: 3}));
        assert_eq!(violates_ko, true);    
    }
    
    #[test]
    fn test_is_over(){
        let mut states = GameState::new(5);
        GameState::apply_move(&mut states, Action::Resign);
        assert_eq!(GameState::is_over(&states), true);

        states = GameState::new(5);
        GameState::apply_move(&mut states, Action::Pass);
        assert_eq!(GameState::is_over(&states), false);
        GameState::apply_move(&mut states, Action::Pass);
        assert_eq!(GameState::is_over(&states), true);

        states = GameState::new(5);
        GameState::apply_move(&mut states, Action::Pass);
        assert_eq!(GameState::is_over(&states), false);
        GameState::apply_move(&mut states, Action::Play(Point{row: 1, col: 1}));
        GameState::apply_move(&mut states, Action::Pass);
        assert_eq!(GameState::is_over(&states), false);
        GameState::apply_move(&mut states, Action::Pass);
        assert_eq!(GameState::is_over(&states), true);
    }

    #[test]
    fn test_self_capture(){
        let mut states = GameState::new(3);
        GameState::apply_move(&mut states, Action::Play(Point{row: 2, col: 1}));
        GameState::apply_move(&mut states, Action::Play(Point{row: 3, col: 3}));
        GameState::apply_move(&mut states, Action::Play(Point{row: 1, col: 2}));
        assert_eq!(states.last().unwrap().board.is_self_capture(Player::White, &Point { row: 1, col: 1 }), true);
        

        GameState::apply_move(&mut states, Action::Play(Point{row: 1, col: 1}));

    }

}