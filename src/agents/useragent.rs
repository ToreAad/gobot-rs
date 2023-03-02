
use super::agent::Agent;
use crate::gamestate::{GameState};
use crate::gotypes::Move;
use crate::{gotypes::{Point}};




pub struct UserAgent{}

impl Agent for UserAgent{
    fn select_move(&self, _: &Vec<GameState>) -> Move {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let command = input.next().unwrap();
        match command {
            "play" => {
                let rowAlpha = input.next().unwrap().chars().next().unwrap();
                let col = rowAlpha as i32 - 'a' as i32 + 1;
                let row = input.next().unwrap().parse::<i32>().unwrap();
                Move::Play(Point{row, col})
            }
            "pass" => Move::Pass,
            "resign" => Move::Resign,
            _ => panic!("Invalid command"),
        }
    }
}
