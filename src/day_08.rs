use std::collections::HashMap;

use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    println!("STUB");
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

fn parse_input(fname: &str) {
    let lines = get_lines(fname);
    let mut row_count = 0;
    let mut col_count = None;
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, line) in lines.enumerate() {
        row_count = i;
        let chars: Vec<char> = line.unwrap().chars().collect();
        if let None = col_count {
            col_count = Some(chars.len());
        }
        for (j,c) in chars.iter().enumerate(){
            match c {
                '.' => (),
                c => {
                    match antennas.get_mut(c) {
                        Some(v) => {
                            v.push((i, j));
                        }
                        None => {
                            antennas.insert(*c, vec![(i, j)]);
                        }
                    }
                }
            }
        }
    }
}