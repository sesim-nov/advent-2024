use grid::Grid;

pub fn part_01(fname: &str) {
    println!("STUB");
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

/// Find the count of times the string XMAS appears in a grid. 
fn find_xmas_count(g: Grid<char>) -> usize {
    let directions: [(isize, isize); 9] = [(0,-1), (0,0), (0,1), (1,-1), (1,0), (1,1), (-1,-1), (-1,0), (-1,1)];
    let mut total: usize = 0;
    for (pos, x) in g.indexed_iter() {
        if *x == 'X' {
            total += directions.iter().map(|dir| look_for_xmas(pos, *dir, "X", &g)).sum::<usize>();
        }
    }
    total
}

fn look_for_xmas(pos: (usize, usize), dir: (isize, isize), sequence: &str, g: &Grid<char>) -> usize {
    let mut sequence = String::from(sequence);
    let mut curr_row = pos.0;
    let mut curr_col = pos.1;
    loop {
        curr_row = match curr_row.checked_add_signed(dir.0){
            Some(e) => e,
            None => {break 0}
        };
        curr_col = match curr_col.checked_add_signed(dir.1){
            Some(e) => e,
            None => {break 0}
        };
        match g.get(curr_row, curr_col) {
            None => {break 0},
            Some(ch) => sequence.push(*ch),
        }
        match sequence.as_str() {
            "X" => continue,
            "XM" => continue,
            "XMA" => continue,
            "XMAS" => break 1,
            _ => break 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;

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
}