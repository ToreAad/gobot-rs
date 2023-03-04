use std::collections::VecDeque;

use sgf_parse::SgfNode;
use sgf_parse::go::{parse, Prop};
use sgf_parse::go::Move as SgfMove;

use crate::game::action::Action;
use crate::game::player::Player;
use crate::game::point::Point;

// use sgf_parse::go::Point as SgfPoint;
// use sgf_parse::GameTree::GoGame;


// use crate::{goboard::Board, gotypes::{Point, Player, Move}};

pub fn get_move(node: &SgfNode<Prop>) -> Option<(Action, Player)>{
    for prop in node.properties(){
        match prop{
            Prop::B(m) => {
                match m{
                    SgfMove::Move(p) => {
                        let y = 1 + p.x as i32;
                        let x = 1 + p.y as i32;
                        if x == 20 && y == 20{
                            // https://www.red-bean.com/sgf/go.html
                            return Some((Action::Pass, Player::Black));
                        }
                        return Some((Action::Play(Point::new(x, y)), Player::Black));
                    },
                    SgfMove::Pass => {
                        return Some((Action::Pass, Player::Black));
                    },
                }
            },
            Prop::W(m) => {
                match m{
                    SgfMove::Move(p) => {
                        let y = 1 + p.x as i32;
                        let x = 1 + p.y as i32;
                        if x == 20 && y == 20{
                            // https://www.red-bean.com/sgf/go.html
                            return Some((Action::Pass, Player::Black));
                        }
                        return Some((Action::Play(Point::new(x, y)), Player::White));
                    },
                    SgfMove::Pass => {
                        return Some((Action::Pass, Player::White));
                    },
                }
            },
            _ => continue,
        }
    }
    None
}

pub fn get_branches(node: &SgfNode<Prop>) -> Vec<VecDeque<(Action, Player)>>{
     let _move = get_move(node);

     let mut collection: Vec<VecDeque<(Action, Player)>> = Vec::new();
    for child in node.children(){
        let res = get_branches(child);
        collection.extend(res);
    }
    if collection.len() == 0{
        collection.push(VecDeque::new());
    } 
    if let Some(m) = _move{
        for branch in collection.iter_mut(){
            branch.push_front(m.clone());
        }
    }
    collection
}


pub fn get_moves(sgf: &str) -> Vec<VecDeque<(Action, Player)>>{

    let gametrees = parse(sgf).unwrap();
    let mut collection: Vec<VecDeque<(Action, Player)>> = Vec::new();
    
    for gt in gametrees{
        let res = get_branches(&gt);
        collection.extend(res);
    }
    // collection
    collection
}
// See https://docs.rs/sgf-parse/latest/sgf_parse/struct.SgfNode.html

#[cfg(test)]
mod test{
    use sgf_parse::GameType;

    use super::*;

    #[test]
    fn test_parse() {
        let sgf = "(;SZ[9]C[Some comment];B[de];W[fe])(;B[de];W[ff])";
        let gametrees = parse(sgf).unwrap();
        assert!(gametrees.len() == 2);
    }
}