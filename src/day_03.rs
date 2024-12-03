use std::ops::Range;

use regex::{Captures, Regex};

use crate::read_file::read_to_string;

pub fn part_01(fname: &str) {
    println!("STUB");
    let memory = read_to_string(fname).unwrap();
    let needles = apply_mul_regex(&memory);
    let total: usize = needles
        .into_iter()
        .map(|x| -> usize { mul_on_capture(x) })
        .sum();
    println!("Part 1: {}", total)
}

pub fn part_02(fname: &str) {
    let memory = read_to_string(fname).unwrap();
    let needles = apply_mul_regex(&memory);
    let inac = get_inactive_ranges(apply_do_regex(&memory));
    let total: usize = needles
        .into_iter()
        .map(|x| -> usize { checked_mul_on_capture(x, &inac) })
        .sum();
    println!("Part 2: {}", total)
}

fn apply_mul_regex(hay: &str) -> Vec<Captures> {
    apply_regex(hay, r"mul\(([0-9]+),([0-9]+)\)")
}

fn apply_do_regex(hay: &str) -> Vec<Captures> {
    apply_regex(hay, r"do(?:n't)?\(\)")
}

fn apply_regex<'r>(hay: &'r str, re: &str) -> Vec<Captures<'r>> {
    let re = Regex::new(re).expect("Regex compilation failed.");
    re.captures_iter(hay).collect()
}

fn get_inactive_ranges(caps: Vec<Captures>) -> Vec<Range<usize>> {
    let mut disabled = false;
    let mut last_change = 0;
    let mut out = Vec::new();
    for cap in caps {
        match cap.get(0).unwrap().as_str() {
            "don't()" => {
                if !disabled {
                    disabled = true;
                    last_change = cap.get(0).unwrap().start()
                }
            }
            _ => {
                if disabled {
                    disabled = false;
                    let end = cap.get(0).unwrap().end();
                    out.push(last_change..end)
                }
            }
        }
    }
    out
}

fn mul_on_capture(m: Captures) -> usize {
    let lhs: usize = m.get(1).unwrap().as_str().parse().unwrap();
    let rhs: usize = m.get(2).unwrap().as_str().parse().unwrap();
    lhs * rhs
}

fn checked_mul_on_capture(m: Captures, inac: &Vec<Range<usize>>) -> usize {
    let is_disabled = inac
        .iter()
        .any(|x| -> bool { x.contains(&m.get(0).unwrap().start()) });
    if !is_disabled {
        mul_on_capture(m)
    } else {
        0
    }
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

    #[test]
    fn test_get_inactive_ranges() {
        //Arrange
        let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        //Act
        let res = apply_do_regex(hay);
        println!("{:?}", res);
        let inac = get_inactive_ranges(res);
        println!("{:?}", inac);
        //Assert
        assert_eq!(inac, vec![20..63]);
    }
}
