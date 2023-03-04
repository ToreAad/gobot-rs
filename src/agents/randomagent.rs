use super::agent::Agent;
use rand::{thread_rng, Rng};
use crate::game::game::Game;
use crate::game::action::Action;

pub struct RandomAgent{}

impl Agent for RandomAgent{
    fn select_action(&self, game_states: &dyn Game) -> Action{
        let candidates = game_states.legal_moves();
        if candidates.len() == 0{
            return Action::Pass;
        }
        let mut rng = thread_rng();
        candidates[rng.gen_range(0..candidates.len())]
    }
}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use super::*;
    use crate::{game::player::Player, go::go::Go};

    #[test]
    fn test_random_agent()
    {
        let board_size = 2;
        let mut game = Go::new(board_size);
    
        let bots = HashMap::from([
            (Player::Black, RandomAgent{}),
            (Player::White, RandomAgent{}),
        ]);
    
        loop {
            print!("{}[2J", 27 as char);
            print!("{}", game);
            if game.is_over() {
                break;
            }
    
            let player = game.current_player();
            let agent = bots.get(&player).unwrap();
            let action = agent.select_action(&game);
            game.apply_move(action);
        }
    }
}