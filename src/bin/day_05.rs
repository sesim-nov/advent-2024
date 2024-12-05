
use advent_2024::day_04::{part_01, part_02};

fn main() {
    part_01("input/04.txt");
    part_02("input/04.txt");
}

struct Rule {
    left: usize, 
    right: usize,
}

impl TryFrom<&str> for Rule {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut spl = s.split("|");
        let left = match spl.next() {
            None => return Err("Failed to split"),
            Some(i) => {
                match i.parse::<usize>() {
                    Err(_) => return Err("Failed to parse left int"),
                    Ok(j) => j,
                }
            }
        };
        let right = match spl.next() {
            None => return Err("Failed to split"),
            Some(i) => {
                match i.parse::<usize>() {
                    Err(_) => return Err("Failed to parse right int"),
                    Ok(j) => j,
                }
            }
        };
        Ok(Self{
            left,
            right
        })
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_parse() {
        //Arrange
        let s1 = "51|49";
        let s2 = "38|potato";
        //Act
        let r1 = Rule::try_from(s1);
        let r2 = Rule::try_from(s2);
        //Assert
        assert!(r1.is_ok());
        assert!(r2.is_err());
        let r1 = r1.unwrap();
        assert_eq!(r1.left, 51);
        assert_eq!(r1.right, 49);
    }
}