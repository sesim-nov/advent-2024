use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    println!("STUB");
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

struct Position((usize, usize));

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    pos: (usize, usize),
    dir: Direction,
}

impl Guard {
    fn new() -> Self {
        Self {
            pos: (0,0),
            dir: Direction::Left,
        }
    }
    fn update(&mut self, pos: (usize, usize), dir: Direction) {
        self.pos = pos;
        self.dir = dir;
    }
}

struct Map {
    bounds: (usize, usize),
    obstacles: Vec<(usize, usize)>,
    guard: Guard,
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
