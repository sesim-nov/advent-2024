use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    println!("STUB");
}

pub fn part_02(fname: &str) {
    println!("STUB");
}

struct Equation {
    check_val: usize, 
    constants: Vec<usize>,
}

impl Equation {
    fn from_str(s: &str) -> Self {
        let mut spl = s.split(':');
        let check_val: usize = spl.next().unwrap().parse().unwrap();
        let constants: Vec<usize> = spl.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        Self { check_val, constants}
    }
}

fn parse_input(fname: &str) -> Vec<Equation> {
    let mut equations = Vec::new();
    let lines = get_lines(fname);
    for line in lines{
        equations.push(Equation::from_str(line.unwrap().as_str()));
    }
    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        //Arrange
        //Act
        let res = parse_input("test_input/07.txt");
        //Assert
        assert_eq!(res.get(0).unwrap().check_val, 190);
        assert_eq!(res.get(8).unwrap().check_val, 292);
    }
}