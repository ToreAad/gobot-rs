use std::{collections::HashMap, thread, time, io::Read};
use std::fs::File;
use std::io::prelude::*;


mod gamestate;
use gamestate::GameState;
mod gotypes;

mod goboard;
mod scoring;
mod agents;
use agents::agent::{Agent};
use agents::randomagent::RandomAgent;
use agents::useragent::UserAgent;
use agents::mcagent::McAgent;
mod sgf;

use crate::scoring::GameResult;
mod gostring;

mod zobrist;


fn main() {
    let board_size = 5;

    let mut game = GameState::new(board_size);
    let mut bots: HashMap<gotypes::Player, Box<dyn Agent>> = HashMap::new();
    bots.insert(gotypes::Player::Black, Box::new(RandomAgent{}));
    bots.insert(gotypes::Player::White, Box::new(RandomAgent{}));

    loop {
        print!("{}[2J", 27 as char);
        thread::sleep(time::Duration::from_millis(100));

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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_sgf(){
        // https://gtl.xmp.net/reviews/eidogo/player.php?sgf=/sgf/2/215-garm-ManyFaces-ziv.sgf
        let fp = "./data/215-garm-ManyFaces-ziv.sgf";
        let mut file = std::fs::File::open(fp).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
    
    
        let moves = sgf::get_moves(&data);
        let mut game = GameState::new(9);
        for (i, (m, p)) in moves[0].iter().enumerate(){
            GameState::apply_move(&mut game, m.clone());
            print!("{}[2J", 27 as char);
            let current_game = game.last().unwrap();
            print!("{}\n", current_game);
            print!("Move number: {}\n", i+1);
        }
    }
}