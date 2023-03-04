use super::player::Player;

pub trait Score: std::fmt::Display{
    fn score(&self, player: Player) -> f32;
}