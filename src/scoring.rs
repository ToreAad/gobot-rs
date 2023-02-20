use std::{collections::{HashMap, HashSet}, fmt};

use crate::{
    goboard::Board,
    gotypes::{Player, Point},
};

pub struct GameResult {
    pub black_score: f32,
    pub white_score: f32,
    pub komi: f32,
}

impl fmt::Display for GameResult{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = self.white_score + self.komi;
        if self.black_score > w{
            write!(f, "Black wins by {} points", self.black_score - w)
        } else{
            write!(f, "White wins by {} points", w - self.black_score)
        }
    }
}

impl GameResult {
    pub fn new(board: &Board, komi: f64) -> GameResult {
        let territory_map = evaluate_territory(board);
        let territory = Territory::new(territory_map);
        GameResult {
            black_score: territory.black_score(),
            white_score: territory.white_score(),
            komi: komi as f32,
        }
    }

    pub fn winning_margin(&self) -> f32 {
        let w = self.white_score + self.komi;
        (self.black_score - w).abs()
    }
}

pub struct Territory {
    pub num_black_territory: u8,
    pub num_white_territory: u8,
    pub num_black_stones: u8,
    pub num_white_stones: u8,
    pub num_dame: u8,
    pub dame_points: Vec<Point>,
}

impl Territory {
    fn new(territory_map: HashMap<Point, TerritoryState>) -> Territory {
        let mut num_black_territory = 0;
        let mut num_white_territory = 0;
        let mut num_black_stones = 0;
        let mut num_white_stones = 0;
        let mut num_dame = 0;
        let mut dame_points = Vec::new();

        for (point, territory_state) in territory_map {
            match territory_state {
                TerritoryState::BlackTerritory => num_black_territory += 1,
                TerritoryState::WhiteTerritory => num_white_territory += 1,
                TerritoryState::Black => num_black_stones += 1,
                TerritoryState::White => num_white_stones += 1,
                TerritoryState::Dame => {
                    num_dame += 1;
                    dame_points.push(point);
                }
            }
        }

        Territory {
            num_black_territory,
            num_white_territory,
            num_black_stones,
            num_white_stones,
            num_dame,
            dame_points,
        }
    }

    fn black_score(&self) -> f32 {
        self.num_black_territory as f32 + self.num_black_stones as f32
    }

    fn white_score(&self) -> f32 {
        self.num_white_territory as f32 + self.num_white_stones as f32
    }
}

enum TerritoryState {
    Black,
    BlackTerritory,
    White,
    WhiteTerritory,
    Dame,
}

fn stone_to_territory_state(stone: Player) -> TerritoryState {
    match stone {
        Player::Black => TerritoryState::Black,
        Player::White => TerritoryState::White,
    }
}

fn evaluate_territory(board: &Board) -> HashMap<Point, TerritoryState> {
    let mut status = HashMap::new();

    for r in 1..board.num_rows + 1 {
        for c in 1..board.num_cols {
            let point = Point::new(r, c);
            if status.contains_key(&point) {
                continue;
            }
            match board.get(&point) {
                Some(stone) => {
                    status.insert(point, stone_to_territory_state(stone));
                }
                None => {
                    let mut visited = HashSet::new();
                    let (group, neighbour) = collect_region(&point, board, &mut visited);

                    if neighbour.len() == 1 {
                        match neighbour.into_iter().next().unwrap() {
                            Player::Black => {
                                for point in group {
                                    status.insert(point, TerritoryState::BlackTerritory);
                                }
                            }
                            Player::White => {
                                for point in group {
                                    status.insert(point, TerritoryState::WhiteTerritory);
                                }
                            }
                        }
                    } else {
                        for point in group {
                            status.insert(point, TerritoryState::Dame);
                        }
                    }
                }
            };
        }
    }
    status
}

fn collect_region(
    start_pos: &Point,
    board: &Board,
    visited: &mut HashSet<Point>,
) -> (Vec<Point>, HashSet<Player>) {
    if visited.contains(start_pos) {
        return (Vec::new(), HashSet::new());
    }

    let mut all_points = vec![start_pos.clone()];
    let mut all_boarders = HashSet::new();
    visited.insert(start_pos.clone());

    let here = board.get(start_pos);
    let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (delta_r, delta_c) in deltas {
        let next_point = Point::new(start_pos.row + delta_r, start_pos.col + delta_c);
        if !board.is_on_grid(&next_point) {
            continue;
        }
        let neighbour = board.get(&next_point);
        match neighbour {
            Some(stone) => {
                if here == neighbour {
                    let (mut points, borders) = collect_region(&next_point, board, visited);
                    all_points.append(&mut points);
                    all_boarders.extend(borders);
                } else {
                    all_boarders.insert(stone);
                }
            }
            None => continue,
        }
    }

    (all_points, all_boarders)
}
