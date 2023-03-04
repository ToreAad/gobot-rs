
use super::agent::Agent;
use crate::game::game::Game;
use crate::game::action::Action;
use crate::game::point::Point;




pub struct UserAgent{}

impl Agent for UserAgent{
    fn select_action(&self, _: &dyn Game) -> Action{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let command = input.next().unwrap();
        match command {
            "play" => {
                let row_alpha = input.next().unwrap().chars().next().unwrap();
                let col = row_alpha as i32 - 'a' as i32 + 1;
                let row = input.next().unwrap().parse::<i32>().unwrap();
                Action::Play(Point{row, col})
            }
            "pass" => Action::Pass,
            "resign" => Action::Resign,
            _ => panic!("Invalid command"),
        }
    }
}
