use std::{collections::HashSet};

use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    println!("Part 1: {}", solve_part_1(fname));
}

pub fn part_02(_fname: &str) {
    println!("STUB");
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up    => (-1, 0),
            Direction::Down  => ( 1, 0),
            Direction::Left  => ( 0,-1),
            Direction::Right => ( 0, 1),
        }
    }
}

#[derive(PartialEq, Eq)]
enum TravelStatus {
    Continue,
    Exit,
    Cycle,
}

#[derive(Clone)]
struct Guard {
    started: bool,
    start_pos: (usize, usize),
    start_dir: Direction,
    pos: (usize, usize),
    dir: Direction,
    move_history: HashSet<(usize, usize)>,
}

impl Guard {
    fn new() -> Self {
        Self {
            started: false,
            start_pos: (0,0),
            start_dir: Direction::Left,
            pos: (0,0),
            dir: Direction::Left,
            move_history: HashSet::new(),
        }
    }
    fn update(&mut self, pos: (usize, usize), dir: Direction) {
        self.start_pos = pos;
        self.start_dir = dir;
        self.pos = pos;
        self.dir = dir;
        self.move_history.clear();
        self.move_history.insert(pos);
    }
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up    => Direction::Right,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn move_to(&mut self, new_pos: (usize, usize)) {
        self.pos = new_pos;
        self.move_history.insert(new_pos);
        if !self.started {
            self.started = true;
        }
    }
}

#[derive(Clone)]
struct Map {
    bounds: (usize, usize),
    obstacles: Vec<(usize, usize)>,
    guard: Guard,
}

impl Map {
    fn move_guard(&mut self) -> TravelStatus {
        loop{
            let proposed_dir = self.guard.dir.get_tuple();
            let proposed_next_r = match self.guard.pos.0.checked_add_signed(proposed_dir.0) {
                Some(i) => i,
                None => break TravelStatus::Exit,
            };
            let proposed_next_c = match self.guard.pos.1.checked_add_signed(proposed_dir.1) {
                Some(i) => i,
                None => break TravelStatus::Exit,
            };
            if proposed_next_r >= self.bounds.0 || proposed_next_c >= self.bounds.1 {
                break TravelStatus::Exit;
            } else if self.guard.pos == self.guard.start_pos && self.guard.dir == self.guard.start_dir && self.guard.started {
                break TravelStatus::Cycle;
            } else {
                if self.obstacles.iter().any(|x| x.0 == proposed_next_r && x.1 == proposed_next_c) {
                    self.guard.turn_right();
                    continue;
                } else {
                    self.guard.move_to((proposed_next_r, proposed_next_c));
                    break TravelStatus::Continue;
                }
            }
        }
    }
    fn insert_obstacle(&mut self, new_obstacle: (usize, usize)) {
        self.obstacles.push(new_obstacle);
    }
    fn get_num_visited(&mut self) -> Option<usize> {
        let mut status = TravelStatus::Continue;
        while status == TravelStatus::Continue {
            status = self.move_guard();
        }
        if status == TravelStatus::Exit {
            Some(self.guard.move_history.len())
        } else {
            None
        }
    }
}

fn solve_part_1(fname: &str) -> usize {
    let mut board = parse_game_board(fname);
    board.get_num_visited().unwrap()
}

fn solve_part_2(fname: &str) -> usize {
    let base_board = parse_game_board(fname);
    let mut valid_locations = 0;
    for r in 0..base_board.bounds.0 {
        for c in 0..base_board.bounds.1 {
            let mut cloned_board = base_board.clone();
            cloned_board.insert_obstacle((r,c));
            if let None = cloned_board.get_num_visited() {
                valid_locations += 1;
            }
        }
    }
    valid_locations
}

fn parse_game_board(fname: &str) -> Map {
    let mut guard = Guard::new();
    let lines: Vec<_> = get_lines(fname).collect();
    let row_count = lines.len();
    let mut col_count: Option<usize> = None;
    let mut obstacles: Vec<(usize, usize)> = Vec::new();
    for (r, line) in lines.into_iter().enumerate() {
        let line = line.unwrap();
        let chs = line.chars();
        if let None = col_count {
            col_count = Some(chs.clone().collect::<Vec<char>>().len());
        }
        for (c, ch) in chs.enumerate() {
            match ch {
                '^' => guard.update((r,c), Direction::Up),
                'v' => guard.update((r,c), Direction::Down),
                '<' => guard.update((r,c), Direction::Left),
                '>' => guard.update((r,c), Direction::Right),
                '#' => obstacles.push((r,c)),
                _ => (),
            }
        }
    }
    Map {
        bounds: (row_count, col_count.unwrap()),
        obstacles,
        guard,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        //Arrange
        //Act
        let board = parse_game_board("test_input/06.txt");
        //Assert
        assert_eq!(board.bounds, (10,10));
        assert_eq!(board.guard.pos, (6,4));
        assert!(board.obstacles.contains(&(0,4)));
    }

    #[test]
    fn test_solve() {
        //Arrange
        //Act
        let steps = solve_part_1("test_input/06.txt");
        //Assert
        assert_eq!(steps, 41)
    }

    #[test]
    fn test_cycle_detection() {
        //Arrange
        let guard = Guard {
            started: false,
            start_dir: Direction::Right,
            start_pos: (1,1),
            dir: Direction::Right,
            pos: (1,1),
            move_history: HashSet::new(),
        };
        let mut board = Map {
            bounds: (10,10),
            obstacles: vec![(0,1), (1,9), (8,0), (9,8)],
            guard
        };
        //Act
        let res = board.get_num_visited();
        //Assert
        assert_eq!(None, res);
    }

    #[test]
    fn test_solve_2() {
        //Arrange
        //Act
        let valid_locs = solve_part_2("test_input/06.txt");
        //Assert
        assert_eq!(valid_locs, 6)
    }
}