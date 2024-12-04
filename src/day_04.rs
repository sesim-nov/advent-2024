use grid::Grid;

pub fn part_01(fname: &str) {
    println!("STUB");
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

/// Find the count of times the string XMAS appears in a grid. 
fn find_xmas_count(g: Grid<char>) -> usize {
    for ((r, c), x) in g.indexed_iter() {
        println!("{} {} {}", r, c, x);
    }
    4
}