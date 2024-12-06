use grid::Grid;

use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    let g = read_input(fname);
    let xmases = find_xmas_count(g);
    println!("Part 01: {}", xmases);
}

pub fn part_02(fname: &str) {
    let g = read_input(fname);
    let xmases = find_mas_x_count(g);
    println!("Part 02: {}", xmases);
}

fn read_input(fname: &str) -> Grid<char> {
    let mut lines = get_lines(fname);
    let mut grid_base: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let row_size = grid_base.len();
    for line in lines {
        let mut next_row: Vec<char> = line.unwrap().chars().collect();
        grid_base.append(&mut next_row);
    }
    Grid::from_vec(grid_base, row_size)
}

/// Find the count of times the string XMAS appears in a grid.
fn find_xmas_count(g: Grid<char>) -> usize {
    let directions: [(isize, isize); 9] = [
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut total: usize = 0;
    for (pos, x) in g.indexed_iter() {
        if *x == 'X' {
            total += directions
                .iter()
                .map(|dir| look_for_xmas(pos, *dir, &g))
                .sum::<usize>();
        }
    }
    total
}

fn look_for_xmas(pos: (usize, usize), dir: (isize, isize), g: &Grid<char>) -> usize {
    let mut curr_row = pos.0;
    let mut curr_col = pos.1;
    let mut sequence = g.get(curr_row, curr_col).unwrap().to_string();
    loop {
        curr_row = match curr_row.checked_add_signed(dir.0) {
            Some(e) => e,
            None => break 0,
        };
        curr_col = match curr_col.checked_add_signed(dir.1) {
            Some(e) => e,
            None => break 0,
        };
        match g.get(curr_row, curr_col) {
            None => break 0,
            Some(ch) => sequence.push(*ch),
        }
        match sequence.as_str() {
            "X" => continue,
            "XM" => continue,
            "XMA" => continue,
            "XMAS" => break 1,
            _ => break 0,
        }
    }
}

/// Find the count of times the X-shaped MAS pattern appears in a grid.
fn find_mas_x_count(g: Grid<char>) -> usize {
    let mut total: usize = 0;
    for (pos, x) in g.indexed_iter() {
        if *x == 'A' {
            total += find_mas_x(pos, &g);
        }
    }
    total
}

fn find_mas_x(pos: (usize, usize), g: &Grid<char>) -> usize {
    let valid_ptns = ["SSMM", "MMSS", "MSMS", "SMSM"];
    let quadrants = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let ptn: String = quadrants
        .iter()
        .map(|(dr, dc)| -> char {
            let r = match pos.0.checked_add_signed(*dr) {
                None => return '\0',
                Some(i) => i,
            };
            let c = match pos.1.checked_add_signed(*dc) {
                None => return '\0',
                Some(i) => i,
            };
            match g.get(r, c) {
                None => '\0',
                Some(c) => *c,
            }
        })
        .collect();
    let ptn_str = ptn.as_str();
    if valid_ptns.into_iter().any(|x| x == ptn_str) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;

    #[test]
    fn test_read_input() {
        //Arrange
        let fname = "test_input/04.txt";

        //Act
        let res = read_input(fname);

        //Assert
        assert_eq!('M', *res.get(0, 0).unwrap());
        assert_eq!('S', *res.get(1, 1).unwrap());
        assert_eq!('X', *res.get(2, 2).unwrap());
    }

    #[test]
    fn test_find_xmas() {
        //Arrange
        let test_grid = grid![
            ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']
            ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A']
            ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M']
            ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X']
            ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M']
            ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A']
            ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S']
            ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A']
            ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M']
            ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']];
        //Act
        let res = find_xmas_count(test_grid);

        //Assert
        assert_eq!(res, 18);
    }

    #[test]
    fn test_find_mas_x() {
        //Arrange
        let test_grid = grid![
            ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']
            ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A']
            ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M']
            ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X']
            ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M']
            ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A']
            ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S']
            ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A']
            ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M']
            ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']];
        //Act
        let res = find_mas_x_count(test_grid);

        //Assert
        assert_eq!(res, 9);
    }
}
