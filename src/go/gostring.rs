use std::{collections::{HashSet}};

use crate::game::{player::Player, point::Point};


#[derive(Clone)]
pub struct GoString{
    pub player: Player,
    pub stones: HashSet<Point>,
    pub liberties: HashSet<Point>
}

impl GoString{
    pub fn new(player: Player, stones_vec: Vec<Point>, liberties_vec: Vec<Point>) -> GoString{
        let mut liberties: HashSet<Point> = HashSet::new();
        for l in liberties_vec{
            liberties.insert(l);
        }
        let mut stones = HashSet::new();
        for s in stones_vec{
            stones.insert(s);
        }
        GoString{player, stones: stones, liberties: liberties}
    }

    pub fn add_stone(&mut self, point: Point){
        // TODO: Why dont I use this method?
        self.stones.insert(point);
    }

    pub fn remove_liberty(&mut self, point: &Point){
        self.liberties.remove(point);
    }

    pub fn add_liberty(&mut self, point: &Point){
        self.liberties.insert(point.clone());
    }

    pub fn merged_with(&self, other: &GoString) -> GoString{
        let mut combined_stones = HashSet::new();
        for stone in self.stones.union(&other.stones){
            combined_stones.insert(*stone);
        }
        let mut combined_liberties = HashSet::new();
        for liberty in self.liberties.union(&other.liberties){
            if combined_stones.contains(liberty){
                continue;
            }
            combined_liberties.insert(*liberty);
        }
        GoString { player: self.player, stones: combined_stones, liberties: combined_liberties }
    }

    pub fn num_liberties(&self) -> usize{
        self.liberties.len()
    }
}

impl PartialEq for GoString{
    fn eq(&self, other: &GoString) -> bool{
        self.player == other.player && self.stones == other.stones && self.liberties == other.liberties
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_gosting_new(){
        let player = Player::Black;
        let stones = vec![Point{row: 1, col: 1}, Point{row: 2, col: 2}];
        let liberties = vec![Point{row: 0, col: 0}, Point{row: 1, col: 0}];
        let gosting = GoString::new(player, stones, liberties);
        assert_eq!(gosting.player, Player::Black);
        assert_eq!(gosting.stones.len(), 2);
        assert_eq!(gosting.liberties.len(), 2);
    }

    #[test]
    fn test_gosting_add_stone(){
        let mut gosting = GoString::new(Player::Black, vec![], vec![]);
        gosting.add_stone(Point{row: 1, col: 1});
        assert_eq!(gosting.stones.len(), 1);
    }

    #[test]
    fn test_gosting_remove_liberty(){
        let mut gosting = GoString::new(Player::Black, vec![], vec![Point{row: 1, col: 1}]);
        gosting.remove_liberty(&Point{row: 1, col: 1});
        assert_eq!(gosting.liberties.len(), 0);
    }

    #[test]
    fn test_gosting_add_liberty(){
        let mut gosting = GoString::new(Player::Black, vec![], vec![]);
        gosting.add_liberty(&Point{row: 1, col: 1});
        assert_eq!(gosting.liberties.len(), 1);
    }

    #[test]
    fn test_gosting_merged_with(){
        let mut gosting1 = GoString::new(Player::Black, vec![Point{row: 1, col: 1}], vec![Point{row: 0, col: 0}]);
        let gosting2 = GoString::new(Player::Black, vec![Point{row: 2, col: 2}], vec![Point{row: 1, col: 0}]);
        gosting1 = gosting1.merged_with(&gosting2);
        assert_eq!(gosting1.stones.len(), 2);
        assert_eq!(gosting1.liberties.len(), 2);
    }

    #[test]
    fn test_gosting_complex_merged_with(){
        let mut gosting1 = GoString::new(Player::Black, vec![Point{row: 1, col: 1},Point{row: 1, col: 2}], vec![Point{row: 2, col: 1}]);
        let gosting2 = GoString::new(Player::Black, vec![Point{row: 2, col: 2}], vec![Point{row: 2, col: 1}]);
        gosting1 = gosting1.merged_with(&gosting2);
        assert_eq!(gosting1.stones.len(), 3);
        assert_eq!(gosting1.liberties.len(), 1);
    }

}