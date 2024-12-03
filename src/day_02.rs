use crate::read_file::get_lines;

#[derive(Eq, PartialEq)]
enum Slope {
    Increasing,
    Decreasing,
    BadSlope,
}

fn is_safe(record: Vec<usize>) -> bool {
    if record.len() < 2 {
        return true;
    }
    let res_vec: Vec<Slope> = record
        .windows(2)
        .map(|x| -> Slope {
            let left = x[0];
            let right = x[1];
            let comp = right.abs_diff(left);
            if comp < 1 || comp > 3 {
                Slope::BadSlope
            } else if right > left {
                Slope::Increasing
            } else {
                Slope::Decreasing
            }
        })
        .collect();
    // If all enums are the same, and none are OOB, we're good.
    res_vec.windows(2).all(|x| -> bool { x[0] == x[1] })
        && *res_vec.get(0).unwrap() != Slope::BadSlope
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|x| -> usize { x.parse().unwrap() })
        .collect()
}

pub fn part_01(fname: &str) {
    let lines = get_lines(fname);
    let total: usize = lines
        .map(|line| -> usize {
            let record = parse_line(&line.unwrap());
            if is_safe(record) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{}", total);
}

pub fn part_02(fname: &str) {
    let lines = get_lines(fname);
    let total: usize = lines
        .map(|line| -> usize {
            let record = parse_line(&line.unwrap());
            for i in 0..record.len() {
                let mut record_cpy = record.clone();
                record_cpy.remove(i);
                if is_safe(record_cpy) {
                    return 1;
                }
            }
            0
        })
        .sum();
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_fn() {
        //Arrange
        let record_1 = vec![7, 6, 4, 2, 1];
        let record_2 = vec![1, 2, 7, 8, 9];
        let record_3 = vec![1, 3, 2, 4, 5];
        let record_4 = vec![8, 6, 4, 4, 1];

        //Act
        let res_1 = is_safe(record_1);
        let res_2 = is_safe(record_2);
        let res_3 = is_safe(record_3);
        let res_4 = is_safe(record_4);

        //Assert
        assert!(res_1);
        assert!(!res_2);
        assert!(!res_3);
        assert!(!res_4);
    }
}
