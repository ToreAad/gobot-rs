use std::{collections::{HashSet, HashMap}, fmt, cell::RefCell};

use crate::gotypes::{Player, Point};
use crate::gostring::GoString;

#[derive(Clone)]
pub struct Board{
    pub num_rows: i32,
    pub num_cols: i32,
    pub grid: HashMap<Point, RefCell<GoString>>,
}

impl fmt::Display for Board{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.num_rows{
            write!(f, "{} ", row)?;
            for col in 0..self.num_cols{
                let point = Point{row, col};
                match self.grid.get(&point){
                    Some(s) => write!(f, "{}", if s.borrow().player==Player::Black {"X"} else {"O"})?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for c in ('a'..='z').take(self.num_cols as usize){
            write!(f, "{}", c)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Board{
    pub fn new(size: i32) -> Board{
        Board{
            num_rows: size,
            num_cols: size,
            grid: HashMap::new(),
        }
    }

    pub fn place_stone(&mut self, player: Player, point: Point){
        if !self.is_on_grid(&point){
            panic!("Point is not on the grid");
        }
        if self.get(&point).is_some(){
            panic!("Point is already occupied");
        }

        let mut adjacent_same_color = Vec::new();
        let mut adjacent_opposite_color = Vec::new();
        let mut liberties =  Vec::new();

        for neighbour in point.neighbours(){
            if !self.is_on_grid(&neighbour){
                continue;
            }
            let maybe_neighbour_string = self.grid.get(&neighbour);
            match maybe_neighbour_string{
                None => liberties.push(neighbour),
                Some(neighbour_string) => {
                    if neighbour_string.borrow().player == player && !adjacent_same_color.contains(neighbour_string){
                        adjacent_same_color.push(RefCell::clone(neighbour_string));
                    }else if neighbour_string.borrow().player != player && !adjacent_opposite_color.contains(neighbour_string){
                        adjacent_opposite_color.push(RefCell::clone(neighbour_string));
                    }
                }
            }
        }

        let mut new_string = GoString::new(player, vec![point], liberties);

        for same_color_string in &adjacent_same_color{
            new_string = new_string.merged_with(&same_color_string.borrow());
        }
        let new_string_refcell = RefCell::new(new_string.clone());
        for new_string_point in new_string.stones.clone(){
            self.grid.insert(new_string_point, RefCell::clone(&new_string_refcell));
        }

        for other_color_string in &adjacent_opposite_color{
            other_color_string.borrow_mut().remove_liberty(&point);
        }
        for other_color_string in &adjacent_opposite_color{
            if other_color_string.borrow().num_liberties() == 0{
                self.remove_string(&other_color_string.borrow());
            }
        }
    }

    pub fn remove_string(&mut self, string: &GoString){
        for point in string.stones.iter(){
            self.grid.remove(&point);
        }
    }

    pub fn is_on_grid(&self, point: &Point) -> bool{
        point.row >= 1 && point.row <= self.num_rows && point.col >= 1 && point.col <= self.num_cols
    }

    pub fn get_go_string(&self, point: &Point) -> Option<RefCell<GoString>>{
        match self.grid.get(point){
            Some(s) => Some(RefCell::clone(s)),
            None => None,
        }
    }

    pub fn get(&self, point: &Point) -> Option<Player>{
        match self.grid.get(point){
            None => None,
            Some(string) => match string.borrow().player{
                Player::Black => Some(Player::Black),
                Player::White => Some(Player::White),
            }
        }
    }
}

impl PartialEq for Board{
    fn eq(&self, other: &Board) -> bool{
        self.num_rows == other.num_rows && self.num_cols == other.num_cols && self.grid == other.grid
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_place_stone(){
        let mut board = Board::new(2);
        board.place_stone(Player::Black, Point{row: 0, col: 0});
        assert_eq!(board.get(&Point{row: 0, col: 0}), Some(Player::Black));
        assert_eq!(board.get(&Point{row: 1, col: 0}), None);
        assert_eq!(board.get(&Point{row: 1, col: 1}), None);
        assert_eq!(board.get(&Point{row: 0, col: 1}), None);
    }

    #[test]
    fn test_place_two_different_stones(){
        let mut board = Board::new(2);
        board.place_stone(Player::Black, Point{row: 0, col: 0});
        board.place_stone(Player::White, Point{row: 1, col: 0});
        assert_eq!(board.get(&Point{row: 0, col: 0}), Some(Player::Black));
        assert_eq!(board.get(&Point{row: 1, col: 0}), Some(Player::White));
        assert_eq!(board.get(&Point{row: 1, col: 1}), None);
        assert_eq!(board.get(&Point{row: 0, col: 1}), None);
    }

    #[test]
    fn test_place_two_similar_stones(){
        let mut board = Board::new(2);
        board.place_stone(Player::Black, Point{row: 0, col: 0});
        board.place_stone(Player::Black, Point{row: 1, col: 0});
        assert_eq!(board.get(&Point{row: 0, col: 0}), Some(Player::Black));
        assert_eq!(board.get(&Point{row: 1, col: 0}), Some(Player::Black));
        assert_eq!(board.get(&Point{row: 1, col: 1}), None);
        assert_eq!(board.get(&Point{row: 0, col: 1}), None);
    }

    #[test]
    fn test_capture(){
        let mut board = Board::new(19);
        board.place_stone(Player::Black, Point{row: 2, col: 2});
        board.place_stone(Player::White, Point{row: 1, col: 2});
        assert_eq!(board.get(&Point{row: 1, col: 2}), Some(Player::White));
        assert_eq!(board.get(&Point{row: 2, col: 2}), Some(Player::Black));
        board.place_stone(Player::White, Point{row: 2, col: 1});
        assert_eq!(board.get(&Point{row: 2, col: 2}), Some(Player::Black));
        board.place_stone(Player::White, Point{row: 2, col: 3});
        assert_eq!(board.get(&Point{row: 2, col: 2}), Some(Player::Black));
        board.place_stone(Player::White, Point { row: 3, col: 2 });
        assert_eq!(board.get(&Point{row: 2, col: 2}), None);
    }

    #[test]
    fn test_capture_two_stones(){
        let mut board = Board::new(19);
        board.place_stone(Player::Black, Point{row: 2, col: 2});
        board.place_stone(Player::Black, Point{row: 2, col: 3});
        board.place_stone(Player::White, Point{row: 1, col: 2});
        board.place_stone(Player::White, Point{row: 1, col: 3});
        assert_eq!(board.get(&Point{row: 2, col: 2}), Some(Player::Black));
        assert_eq!(board.get(&Point{row: 2, col: 3}), Some(Player::Black));
        board.place_stone(Player::White, Point{row: 3, col: 2});
        board.place_stone(Player::White, Point{row: 3, col: 3});
        assert_eq!(board.get(&Point{row: 2, col: 2}), Some(Player::Black));
        assert_eq!(board.get(&Point{row: 2, col: 3}), Some(Player::Black));
        board.place_stone(Player::White, Point{row: 2, col: 1});
        board.place_stone(Player::White, Point{row: 2, col: 4});
        assert_eq!(board.get(&Point{row: 2, col: 2}), None);
        assert_eq!(board.get(&Point{row: 2, col: 3}), None);
    }

    #[test]
    fn test_capture_is_not_suicide(){
        let mut board = Board::new(19);
        board.place_stone(Player::Black, Point{row: 1, col: 1});
        board.place_stone(Player::Black, Point{row: 2, col: 2});
        board.place_stone(Player::Black, Point{row: 1, col: 3});
        board.place_stone(Player::White, Point{row: 2, col: 1});
        board.place_stone(Player::White, Point{row: 1, col: 2});
        assert_eq!(board.get(&Point{row: 2, col: 2}), None);
        board.place_stone(Player::White, Point{row: 2, col: 1});
        board.place_stone(Player::White, Point{row: 1, col: 2});
        assert_eq!(board.get(&Point{row: 2, col: 1}), Some(Player::White));
        assert_eq!(board.get(&Point{row: 1, col: 2}), Some(Player::White));
    }

    #[test]
    fn test_remove_liberties(){
        let mut board = Board::new(5);
        board.place_stone(Player::Black, Point{row: 3, col: 3});
        board.place_stone(Player::White, Point{row: 2, col: 2});
        let white_string = board.get_go_string(&Point{row: 2, col:2});
        match white_string{
            None => assert!(false),
            Some(string) => {
                assert_eq!(string.borrow().liberties.len(), 4);
            }
        }
        board.place_stone(Player::Black, Point{row: 3, col: 2});
        let white_string = board.get_go_string(&Point{row: 2, col:2});
        match white_string{
            None => assert!(false),
            Some(string) => {
                assert_eq!(string.borrow().liberties.len(), 3);
            }
        }
    }

    #[test]
    fn test_empty_triangle(){
        let mut board = Board::new(5);
        board.place_stone(Player::Black, Point{row: 1, col: 1});
        board.place_stone(Player::Black, Point{row: 1, col: 2});
        board.place_stone(Player::Black, Point{row: 2, col: 2});
        board.place_stone(Player::White, Point{row: 2, col: 1});
        
        let black_string = board.get_go_string(&Point{row: 1, col:1});
        match black_string{
            None => assert!(false),
            Some(string) => {
                let gostring = string.borrow();
                assert_eq!(gostring.liberties.len(), 3);
            }
        }
    }
}
