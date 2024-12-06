mod rule;
use rule::*;

use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    println!("STUB");
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(fname: &str) -> Input{
    let mut lines = get_lines(fname);
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    // Loop parsing rules
    loop {
        let line = match lines.next() {
            None => break,
            Some(l) => l.expect("Failed to get string"),
        };
        let new_rule = match Rule::try_from(line.as_str()) {
            Err(_) => break,
            Ok(r) => r,
        };
        rules.push(new_rule);
    }

    // Loop parsing updates
    loop {
        let line = match lines.next() {
            None => break,
            Some(l) => l.expect("Failed to get string"),
        };
        let mut new_update: Vec<usize> = Vec::new();
        let spl = line.split(",");
        for chunk in spl {
            match chunk.parse::<usize>() {
                Ok(i) => new_update.push(i),
                Err(_) => break,
            }
        }
        updates.push(new_update);
    }

    Input {
        rules,
        updates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_parse() {
        //Arrange
        //Act
        let res = parse_input("test_input/05.txt");
        //Assert
        assert_eq!(res.rules.len(), 21);
        assert_eq!(res.rules.get(0).unwrap().left, 47);
        assert_eq!(res.rules.get(20).unwrap().right, 13);
        assert_eq!(res.updates.len(), 6);
        assert_eq!(*res.updates.get(1).unwrap().get(2).unwrap(), 53);
    }
}