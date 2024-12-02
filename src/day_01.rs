use crate::read_file::get_lines;

pub fn part_01(fname: &str) {
    let res = solve_part_01(parse_input(fname));
    println!("{}", res);    
}

struct ProblemInput {
    left: Vec<usize>,
    right: Vec<usize>,
}

fn parse_input(fname: &str) -> ProblemInput {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in get_lines(fname){
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        left.push(split
            .next()
            .unwrap()
            .parse()
            .unwrap()
        );
        right.push(split
            .next()
            .unwrap()
            .parse()
            .unwrap()
        );
    }
    ProblemInput{
        left,
        right,
    }
}

fn solve_part_01(mut data: ProblemInput) -> usize {
    //sort vectors
    data.left.sort();
    data.right.sort();

    data.left
        .iter()
        .zip(data.right.iter())
        .map(|x| -> usize {
            x.0.abs_diff(*x.1)
        })
        .sum()
}

fn solve_part_02(mut data: ProblemInput) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_01() {
        //Arrange
        let test_input = ProblemInput {
            left:  vec![3, 4, 2, 1, 3, 3],
            right: vec![4, 3, 5, 3, 9, 3],
        };

        //Act
        let res = solve_part_01(test_input);

        //Assert
        assert_eq!(res, 11);
    }
}