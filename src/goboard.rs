use std::{
    collections::{HashMap, HashSet},
    fmt,
    rc::Rc, borrow::Borrow,
};

use crate::{gostring::GoString, zobrist::{ZoobristHash, HashCodes}};
use crate::gotypes::{Player, Point};

#[derive(Clone)]
pub struct Board {
    pub num_rows: i32,
    pub num_cols: i32,
    pub grid: HashMap<Point, Rc<GoString>>,
    _hash: u64,
    _hash_codes: Rc<HashCodes>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 1..self.num_rows+1 {
            write!(f, "{} ", row)?;
            for col in 1..self.num_cols+1 {
                let point = Point { row, col };
                match self.grid.get(&point) {
                    Some(s) => write!(f, "{}", if s.player == Player::Black { "X" } else { "O" })?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for c in ('a'..='z').take(self.num_cols as usize) {
            write!(f, "{}", c)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Board {
    pub fn new(size: i32) -> Board {
        let zoobristhash = ZoobristHash::new();
        Board {
            num_rows: size,
            num_cols: size,
            grid: HashMap::new(),
            _hash: zoobristhash.empty_board,
            _hash_codes: Rc::new(zoobristhash.hash_codes),
        }
    }

    pub fn hash(&self) -> u64 {
        self._hash
    }

    pub fn place_stone(&mut self, player: Player, point: Point) {
        if !self.is_on_grid(&point) {
            panic!("Point is not on the grid");
        }
        if self.get(&point).is_some() {
            panic!("Point is already occupied");
        }

        let mut adjacent_same_color = Vec::new();
        let mut adjacent_opposite_color = Vec::new();
        let mut liberties = Vec::new();

        for neighbour in point.neighbours() {
            if !self.is_on_grid(&neighbour) {
                continue;
            }
            let maybe_neighbour_string = self.grid.get(&neighbour);
            match maybe_neighbour_string {
                None => liberties.push(neighbour),
                Some(neighbour_string) => {
                    if neighbour_string.player == player
                        && !adjacent_same_color.contains(neighbour_string)
                    {
                        adjacent_same_color.push(Rc::clone(neighbour_string));
                    } else if neighbour_string.player != player
                        && !adjacent_opposite_color.contains(neighbour_string)
                    {
                        adjacent_opposite_color.push(Rc::clone(neighbour_string));
                    }
                }
            }
        }

        let mut new_string = GoString::new(player, vec![point], liberties);

        for same_color_string in &adjacent_same_color {
            new_string = new_string.merged_with(&same_color_string);
        }
        let new_string_rc = Rc::new(new_string.clone());
        for new_string_point in new_string.stones {
            self.grid
                .insert(new_string_point, Rc::clone(&new_string_rc));
        }

        // TODO
        let key = (point, Some(player));
        let t = self._hash_codes.get(&key).unwrap();
        self._hash ^= t;

        for other_color_string in &adjacent_opposite_color {
            let mut new_other_color_string = other_color_string.as_ref().clone();
            new_other_color_string.remove_liberty(&point);
            if new_other_color_string.num_liberties() == 0 {
                self.remove_string(&new_other_color_string);
            } else {
                let updated_other_color_string_rc = Rc::new(new_other_color_string.clone());
                for other_color_string_point in new_other_color_string.stones {
                    self.grid.insert(
                        other_color_string_point,
                        Rc::clone(&updated_other_color_string_rc),
                    );
                }
            }
        }
    }

    pub fn remove_string(&mut self, string: &GoString) {
        
        for point in string.stones.iter() {
            let key = (point.clone(), Some(string.player));
            let t = self._hash_codes.get(&key).unwrap();
            self._hash ^= t;
            self.grid.remove(&point);
        }
    }

    pub fn is_on_grid(&self, point: &Point) -> bool {
        point.row >= 1 && point.row <= self.num_rows && point.col >= 1 && point.col <= self.num_cols
    }

    pub fn get_go_string(&self, point: &Point) -> Option<Rc<GoString>> {
        match self.grid.get(point) {
            Some(s) => Some(Rc::clone(s)),
            None => None,
        }
    }

    pub fn get(&self, point: &Point) -> Option<Player> {
        match self.grid.get(point) {
            None => None,
            Some(string) => match string.player {
                Player::Black => Some(Player::Black),
                Player::White => Some(Player::White),
            },
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.num_rows == other.num_rows
            && self.num_cols == other.num_cols
            && self.grid == other.grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_place_stone() {
        let mut board = Board::new(2);
        board.place_stone(Player::Black, Point { row: 1, col: 1 });
        assert_eq!(board.get(&Point { row: 1, col: 1 }), Some(Player::Black));
        assert_eq!(board.get(&Point { row: 2, col: 1 }), None);
        assert_eq!(board.get(&Point { row: 2, col: 2 }), None);
        assert_eq!(board.get(&Point { row: 1, col: 2 }), None);
    }

    #[test]
    fn test_place_two_different_stones() {
        let mut board = Board::new(2);
        board.place_stone(Player::Black, Point { row: 1, col: 1 });
        board.place_stone(Player::White, Point { row: 2, col: 1 });
        assert_eq!(board.get(&Point { row: 1, col: 1 }), Some(Player::Black));
        assert_eq!(board.get(&Point { row: 2, col: 1 }), Some(Player::White));
        assert_eq!(board.get(&Point { row: 2, col: 2 }), None);
        assert_eq!(board.get(&Point { row: 1, col: 2 }), None);
    }

    #[test]
    fn test_place_two_similar_stones() {
        let mut board = Board::new(2);
        board.place_stone(Player::Black, Point { row: 1, col: 1 });
        board.place_stone(Player::Black, Point { row: 2, col: 1 });
        assert_eq!(board.get(&Point { row: 1, col: 1 }), Some(Player::Black));
        assert_eq!(board.get(&Point { row: 2, col: 1 }), Some(Player::Black));
        assert_eq!(board.get(&Point { row: 2, col: 2 }), None);
        assert_eq!(board.get(&Point { row: 1, col: 2 }), None);
    }

    #[test]
    fn test_capture() {
        let mut board = Board::new(19);
        board.place_stone(Player::Black, Point { row: 2, col: 2 });
        board.place_stone(Player::White, Point { row: 1, col: 2 });
        assert_eq!(board.get(&Point { row: 1, col: 2 }), Some(Player::White));
        assert_eq!(board.get(&Point { row: 2, col: 2 }), Some(Player::Black));
        board.place_stone(Player::White, Point { row: 2, col: 1 });
        assert_eq!(board.get(&Point { row: 2, col: 2 }), Some(Player::Black));
        board.place_stone(Player::White, Point { row: 2, col: 3 });
        assert_eq!(board.get(&Point { row: 2, col: 2 }), Some(Player::Black));
        board.place_stone(Player::White, Point { row: 3, col: 2 });
        assert_eq!(board.get(&Point { row: 2, col: 2 }), None);
    }

    #[test]
    fn test_capture_two_stones() {
        let mut board = Board::new(19);
        board.place_stone(Player::Black, Point { row: 2, col: 2 });
        board.place_stone(Player::Black, Point { row: 2, col: 3 });
        board.place_stone(Player::White, Point { row: 1, col: 2 });
        board.place_stone(Player::White, Point { row: 1, col: 3 });
        assert_eq!(board.get(&Point { row: 2, col: 2 }), Some(Player::Black));
        assert_eq!(board.get(&Point { row: 2, col: 3 }), Some(Player::Black));
        board.place_stone(Player::White, Point { row: 3, col: 2 });
        board.place_stone(Player::White, Point { row: 3, col: 3 });
        assert_eq!(board.get(&Point { row: 2, col: 2 }), Some(Player::Black));
        assert_eq!(board.get(&Point { row: 2, col: 3 }), Some(Player::Black));
        board.place_stone(Player::White, Point { row: 2, col: 1 });
        board.place_stone(Player::White, Point { row: 2, col: 4 });
        assert_eq!(board.get(&Point { row: 2, col: 2 }), None);
        assert_eq!(board.get(&Point { row: 2, col: 3 }), None);
    }

    #[test]
    fn test_capture_is_not_suicide() {
        let mut board = Board::new(19);
        board.place_stone(Player::Black, Point { row: 1, col: 1 });
        board.place_stone(Player::Black, Point { row: 2, col: 2 });
        board.place_stone(Player::Black, Point { row: 1, col: 3 });
        board.place_stone(Player::White, Point { row: 2, col: 1 });
        board.place_stone(Player::White, Point { row: 1, col: 2 });
        assert_eq!(board.get(&Point { row: 1, col: 1 }), None);
        assert_eq!(board.get(&Point { row: 2, col: 1 }), Some(Player::White));
        assert_eq!(board.get(&Point { row: 1, col: 2 }), Some(Player::White));
    }

    #[test]
    fn test_remove_liberties() {
        let mut board = Board::new(5);
        board.place_stone(Player::Black, Point { row: 3, col: 3 });
        board.place_stone(Player::White, Point { row: 2, col: 2 });
        let white_string = board.get_go_string(&Point { row: 2, col: 2 });
        match white_string {
            None => assert!(false),
            Some(string) => {
                assert_eq!(string.liberties.len(), 4);
            }
        }
        board.place_stone(Player::Black, Point { row: 3, col: 2 });
        let white_string = board.get_go_string(&Point { row: 2, col: 2 });
        match white_string {
            None => assert!(false),
            Some(string) => {
                assert_eq!(string.liberties.len(), 3);
            }
        }
    }

    #[test]
    fn test_empty_triangle() {
        let mut board = Board::new(5);
        board.place_stone(Player::Black, Point { row: 1, col: 1 });
        board.place_stone(Player::Black, Point { row: 1, col: 2 });
        board.place_stone(Player::Black, Point { row: 2, col: 2 });
        board.place_stone(Player::White, Point { row: 2, col: 1 });

        let black_string = board.get_go_string(&Point { row: 1, col: 1 });
        match black_string {
            None => assert!(false),
            Some(string) => {
                let gostring = string;
                assert_eq!(gostring.liberties.len(), 3);
            }
        }
    }
}
