use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    let res = solve_part_1(fname);
    println!("Part 01: {}", res);
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
    fn validate_check_val_p1(&self) -> bool {
        self.recurse_check_val(0, 0, false)
    }
    fn validate_check_val_p2(&self) -> bool {
        self.recurse_check_val(0, 0, true)
    }
    fn recurse_check_val(&self, acc: usize, pos: usize, include_concat: bool) -> bool {
        if pos >= self.constants.len() {
            acc == self.check_val
        } else {
            let next_val = *self.constants.get(pos).unwrap();
            // Check addition side
            let add = self.recurse_check_val(acc + next_val, pos + 1, include_concat);
            // Check multiplication side
            let mul = self.recurse_check_val(acc * next_val, pos + 1, include_concat);
            // Check concatenation
            let concat = if include_concat {
                let mut new_acc = acc.to_string();
                new_acc.push_str(next_val.to_string().as_str());
                let new_acc_parsed: usize = new_acc.parse().unwrap();
                self.recurse_check_val(new_acc_parsed, pos + 1, include_concat)
            } else {
                false
            };
            add || mul || concat
        }
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

fn solve_part_1(fname: &str) -> usize{
    let eqs = parse_input(fname);
    let mut total = 0;
    for eq in eqs {
        total += if eq.validate_check_val_p1() {
            eq.check_val
        } else {
            0
        };
    }
    total
}

fn solve_part_2(fname: &str) -> usize{
    let eqs = parse_input(fname);
    let mut total = 0;
    for eq in eqs {
        total += if eq.validate_check_val_p2() {
            eq.check_val
        } else {
            0
        };
    }
    total
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

    #[test]
    fn test_checking() {
        //Arrange
        let eq_1 = Equation{
            check_val: 190,
            constants: vec![10, 19],
        };
        let eq_2 = Equation {
            check_val: 7290,
            constants: vec![6, 8, 6, 15],
        };
        //Act
        let res_1 = eq_1.validate_check_val_p1();
        let res_2 = eq_2.validate_check_val_p1();
        //Assert
        assert!(res_1);
        assert!(!res_2);
    }

    #[test]
    fn test_checking_2() {
        //Arrange
        let eq_1 = Equation{
            check_val: 190,
            constants: vec![10, 19],
        };
        let eq_2 = Equation {
            check_val: 7290,
            constants: vec![6, 8, 6, 15],
        };
        //Act
        let res_1 = eq_1.validate_check_val_p2();
        let res_2 = eq_2.validate_check_val_p2();
        //Assert
        assert!(res_1);
        assert!(res_2);
    }

    #[test]
    fn test_part_01() {
        //Arrange
        //Act
        let res = solve_part_1("test_input/07.txt");
        //Assert
        assert_eq!(res, 3749);
    }

    #[test]
    fn test_part_02() {
        //Arrange
        //Act
        let res = solve_part_2("test_input/07.txt");
        //Assert
        assert_eq!(res, 11387);
    }
}