use std::{collections::HashMap, thread, time};

mod gamestate;
use gamestate::GameState;
mod gotypes;

mod goboard;
mod scoring;
mod agent;
use agent::{RandomAgent, Agent};

use crate::scoring::GameResult;
mod gostring;

mod zobrist;


fn main() {
    let board_size = 5;

    let mut game = GameState::new(board_size);
    let bots = HashMap::from([
        (gotypes::Player::Black, RandomAgent{}),
        (gotypes::Player::White, RandomAgent{}),
    ]);

    loop {
        print!("{}[2J", 27 as char);
        thread::sleep(time::Duration::from_millis(1000));
        let current_game = game.last().unwrap();
        print!("{}\n", current_game);
        let result = GameResult::new(&current_game.board, 0.0);
        print!("{}\n", result);
        if GameState::is_over(&game) {
            break;
        }

        let player = current_game.next_player();
        let agent = bots.get(&player).unwrap();
        let move_ = agent.select_move(&game);
        GameState::apply_move(&mut game, move_);
    }
}
