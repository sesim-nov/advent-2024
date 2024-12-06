use std::collections::HashSet;
pub struct Rule {
    pub left: usize, 
    pub right: usize,
}

impl Rule {
    fn new(left: usize, right: usize) -> Self {
        Self {
            left,
            right,
        }
    }
    /// Returns TRUE if rule is valid for the given entry
    fn entry_follows_rule(&self, entry: &usize, prior: &HashSet<usize>) -> bool {
        if *entry == self.left {
            !prior.contains(&self.right)
        } else {
            true
        }
    }

    pub fn vector_follows_rule(&self, v: &Vec<usize>) -> bool {
        let mut prior: HashSet<usize> = HashSet::new();
        for entry in v{
            if !self.entry_follows_rule(&entry, &prior) {
                return false;
            }
            prior.insert(*entry);
        }
        true
    }
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
    fn test_rule_apply() {
        //Arrange
        let rules = [
            Rule::new(1,2),
            Rule::new(2,3),
            Rule::new(3,4),
        ];
        let v1 = vec![1,2,3,4];
        let v2 = vec![1,3,2,4];

        //Act
        let r1 = rules.iter().all(|x| x.vector_follows_rule(&v1));
        let r2 = rules.iter().all(|x| x.vector_follows_rule(&v2));

        //Assert
        assert!(r1);
        assert!(!r2);
    }

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