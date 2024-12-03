use regex::{CaptureMatches, Captures, Match, Regex};

use crate::read_file::read_to_string;

pub fn part_01(fname: &str) {
    println!("STUB");
    let memory = read_to_string(fname).unwrap();
    let needles = apply_mul_regex(&memory);
    let total: usize = needles
        .into_iter()
        .map(|x| -> usize {
            mul_on_capture(x)
        })
        .sum();
    println!("Part 1: {}", total)
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

fn apply_mul_regex(hay: &str) -> Vec<Captures> {
    apply_regex(hay, r"mul\(([0-9]+),([0-9]+)\)")
}

fn apply_regex<'r>(hay: &'r str, re: &str) -> Vec<Captures<'r>> {
    let re = Regex::new(re).expect("Regex compilation failed.");
    re.captures_iter(hay).collect()
}


fn mul_on_capture(m: Captures) -> usize {
    let lhs: usize = m.get(1).unwrap().as_str().parse().unwrap();
    let rhs: usize = m.get(2).unwrap().as_str().parse().unwrap();
    lhs*rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        //Arrange
        let hay = "mul(2,2)mul(25,26)some other crapmul(4,4)";

        //Act
        let res = apply_mul_regex(hay);
        let cap_1 = res.get(1).unwrap();
        let num_1 = cap_1.get(1).unwrap().as_str();
        let num_2 = cap_1.get(2).unwrap().as_str();

        //Assert
        assert_eq!("25", num_1);
        assert_eq!("26", num_2);
    }
}