mod go;
mod agents;
mod game;

use std::{collections::HashMap, thread, time};


use go::go::Go;
use game::player::Player;


use agents::agent::{Agent};
use agents::randomagent::RandomAgent;
// use agents::useragent::UserAgent;
// use agents::mcagent::McAgent;

use crate::game::game::Game;



fn main() {
    let board_size = 5;

    let mut game = Go::new(board_size);

    let mut bots: HashMap<Player, Box<dyn Agent>> = HashMap::new();
    bots.insert(Player::Black, Box::new(RandomAgent{}));
    bots.insert(Player::White, Box::new(RandomAgent{}));

    loop {
        print!("{}[2J", 27 as char);
        thread::sleep(time::Duration::from_millis(100));

        print!("{}\n", game);
        let result = game.score();
        print!("{}\n", result);
        if game.is_over() {
            break;
        }

        let player = game.current_player();
        let agent = bots.get(&player).unwrap();
        let action = agent.select_action(&game);
        game.apply_move(action);
    }
}

#[cfg(test)]
mod tests{
    use crate::go::gamestate::GameState;

    use super::*;
    use go::sgf;
    use std::io::prelude::*;

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