use std::fmt;

use crate::{goboard::Board, gotypes::{Point, Player, Move}};

#[derive(Clone)]
pub struct GameState{
    pub board: Board,
    pub next_player: Player,
    pub last_move: Option<Move>,
}

impl fmt::Display for GameState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to play\n{}", self.next_player, self.board)
    }
}


impl GameState{
    pub fn next_player(&self) -> Player{
        self.next_player
    }

    pub fn is_over(states: &Vec<GameState>) -> bool{
        let last_state = &states[states.len()-1];

        match last_state.last_move{
            None => false,
            Some(Move::Play(_)) => false,
            Some(Move::Resign) => true,
            Some(Move::Pass) => {
                match states.get(states.len()-2){
                    None => false,
                    Some(state) => match state.last_move{
                        None => false,
                        Some(Move::Pass) => true,
                        _ => false,
                    }
                }
            }
        }
    }
    pub fn situation(&self) -> (Player, u64){
        (self.next_player, self.board.hash())
    }

    pub fn does_move_violate_ko(states: &Vec<GameState>, move_: Move) -> bool{
        let point = match move_{
            Move::Play(point) => {
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

    pub fn is_valid_move(states: &Vec<GameState>, move_: Move) -> bool{
        let last_state = &states[states.len()-1];
        let board = &last_state.board;
        let player = last_state.next_player.other();
        if GameState::is_over(states){
            return false;
        }
        match move_{
            Move::Pass => {
                return true;
            }
            Move::Resign => {
                return true;
            }
            Move::Play(point) => {
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
        let mut new_board = board.clone();
        new_board.place_stone(player, point);
        let new_string = new_board.grid.get(&point).unwrap();
        new_string.num_liberties() == 0
    }

    pub fn apply_move(states: &mut Vec<GameState>, move_: Move){
        let prev_state = states.get(states.len()-1).unwrap();

        let mut next_state = GameState{
            board: prev_state.board.clone(),
            next_player: prev_state.next_player.other(),
            last_move: Some(move_.clone()),
        };
        match move_{
            Move::Play(point) => {
                next_state.board.place_stone(prev_state.next_player, point);
            }
            Move::Pass => {
                next_state.next_player = prev_state.next_player.other();
            }
            Move::Resign => {
                next_state.next_player = prev_state.next_player.other();
            }
        }
        states.push(next_state);
    }

    pub fn new(board_size: i32) -> Vec<GameState>{
        vec![GameState{
            board: Board::new(board_size),
            next_player: Player::Black,
            last_move: None,
        }]
    }
}

pub fn is_point_an_eye(board: &Board, point: Point, player: Player) -> bool{
    if !board.get(&point).is_none(){
        return false;
    }
    for n in point.neighbours(){
        match board.get(&n){
            None => continue,
            Some(other_player) => {
                if other_player != player{
                    return false;
                }
            }
        }
    }
    let mut friendly_corners = 0;
    let mut off_board_corners = 0;
    let corners = vec![
        Point{row: point.row - 1, col: point.col - 1},
        Point{row: point.row - 1, col: point.col + 1},
        Point{row: point.row + 1, col: point.col - 1},
        Point{row: point.row + 1, col: point.col + 1},
    ];
    for corner in corners.iter(){
        if board.is_on_grid(&corner){
            match board.get(corner){
                None => continue,
                Some(other_player) => {
                    if other_player == player{
                        friendly_corners += 1;
                    }
                }
            }
        } else{
            off_board_corners += 1;
        }

    }

    if off_board_corners > 0{
        return (off_board_corners+friendly_corners) == 4;
    }
    friendly_corners >= 3
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_is_point_an_eye(){
        let mut board = Board::new(5);
        board.place_stone(Player::Black, Point{row: 1, col: 1});
        board.place_stone(Player::Black, Point{row: 1, col: 2});
        board.place_stone(Player::Black, Point{row: 2, col: 3});
        board.place_stone(Player::Black, Point{row: 3, col: 3});
        board.place_stone(Player::Black, Point{row: 4, col: 3});
        board.place_stone(Player::Black, Point{row: 5, col: 3});

        board.place_stone(Player::White, Point{row: 2, col: 1});
        board.place_stone(Player::White, Point{row: 2, col: 2});
        board.place_stone(Player::White, Point{row: 3, col: 2});
        board.place_stone(Player::White, Point{row: 4, col: 2});
        board.place_stone(Player::White, Point{row: 4, col: 1});
        board.place_stone(Player::White, Point{row: 5, col: 2});

        assert_eq!(is_point_an_eye(&board, Point{row: 3, col: 1}, Player::White), true);
        assert_eq!(is_point_an_eye(&board, Point{row: 5, col: 1}, Player::White), true);
        assert_eq!(is_point_an_eye(&board, Point{row: 3, col: 1}, Player::Black), false);
        assert_eq!(is_point_an_eye(&board, Point{row: 5, col: 1}, Player::Black), false);

        assert_eq!(is_point_an_eye(&board, Point{row: 5, col: 5}, Player::Black), false);
        assert_eq!(is_point_an_eye(&board, Point{row: 5, col: 5}, Player::White), false);
    }


    #[test]
    fn test_does_move_violate_ko(){
        let mut states = GameState::new(5);
        GameState::apply_move(&mut states, Move::Play(Point{row: 1, col: 3}));
        GameState::apply_move(&mut states, Move::Play(Point{row: 3, col: 2}));
        GameState::apply_move(&mut states, Move::Play(Point{row: 2, col: 2}));
        GameState::apply_move(&mut states, Move::Play(Point{row: 3, col: 4}));
        GameState::apply_move(&mut states, Move::Play(Point{row: 2, col: 4}));
        let mut violates_ko = GameState::does_move_violate_ko(&states, Move::Play(Point{row: 4, col: 3}));
        assert_eq!(violates_ko, false);
        GameState::apply_move(&mut states, Move::Play(Point{row: 4, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Move::Play(Point{row: 3, col: 3}));
        assert_eq!(violates_ko, false);
        GameState::apply_move(&mut states, Move::Play(Point{row: 3, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Move::Play(Point{row: 2, col: 3}));
        assert_eq!(violates_ko, false);
        GameState::apply_move(&mut states, Move::Play(Point{row: 2, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Move::Play(Point{row: 3, col: 3}));
        assert_eq!(violates_ko, true);
        GameState::apply_move(&mut states, Move::Play(Point{row: 3, col: 3}));
        violates_ko = GameState::does_move_violate_ko(&states, Move::Play(Point{row: 2, col: 3}));
        assert_eq!(violates_ko, true);    
    }

    #[test]
    fn test_is_valid_move(){
        let mut states = GameState::new(2);
        GameState::apply_move(&mut states, Move::Play(Point{row: 1, col: 1}));
        assert_eq!(GameState::is_valid_move(&states, Move::Play(Point{row: 1, col: 1})), false);
        GameState::apply_move(&mut states, Move::Play(Point{row: 1, col: 2}));
        GameState::apply_move(&mut states, Move::Play(Point{row: 2, col: 1}));
        assert_eq!(GameState::is_valid_move(&states, Move::Play(Point{row: 2, col: 2})), false);
    }

    #[test]
    fn test_is_over(){
        let mut states = GameState::new(5);
        GameState::apply_move(&mut states, Move::Resign);
        assert_eq!(GameState::is_over(&states), true);

        states = GameState::new(5);
        GameState::apply_move(&mut states, Move::Pass);
        assert_eq!(GameState::is_over(&states), false);
        GameState::apply_move(&mut states, Move::Pass);
        assert_eq!(GameState::is_over(&states), true);

        states = GameState::new(5);
        GameState::apply_move(&mut states, Move::Pass);
        assert_eq!(GameState::is_over(&states), false);
        GameState::apply_move(&mut states, Move::Play(Point{row: 1, col: 1}));
        GameState::apply_move(&mut states, Move::Pass);
        assert_eq!(GameState::is_over(&states), false);
        GameState::apply_move(&mut states, Move::Pass);
        assert_eq!(GameState::is_over(&states), true);
    }

}