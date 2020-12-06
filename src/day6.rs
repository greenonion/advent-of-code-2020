use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub struct Group {
    answers: Vec<HashSet<u8>>,
}

impl Group {
    pub fn new(answers: &str) -> Group {
        let mut all_answers = Vec::new();
        for answer in answers.split("\n").map(|x| String::from(x)) {
            let mut chars:  HashSet<u8> = HashSet::new();
            for byte in answer.bytes() {
                chars.insert(byte);
            }
            all_answers.push(chars);
        }
        

        Group {
            answers: all_answers
        }
    }

    pub fn anyone_positive(&self) -> u32 {
        let common = self.answers.iter()
            .fold(HashSet::new(), |acc, x| acc.union(&x).map(|x| *x).collect());

        common.len() as u32
    }

    pub fn everyone_positive(&self) -> u32 {
        let first = self.answers[0].clone();
        let common = self.answers[1..].iter()
            .fold(first, |acc, x| acc.intersection(&x).map(|x| *x).collect());

        common.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_answers() {
        let group = Group::new("abc");
        assert_eq!(group.anyone_positive(), 3);

        let group = Group::new("a\nb\nc");
        assert_eq!(group.anyone_positive(), 3);

        let group = Group::new("ab\nac");
        assert_eq!(group.anyone_positive(), 3);

        let group = Group::new("a\na\na\na");
        assert_eq!(group.anyone_positive(), 1);

        let group = Group::new("b");
        assert_eq!(group.anyone_positive(), 1);
    } 
}

pub struct Problem {
    data: Vec<Group>,
}

impl Problem {
    pub fn new() -> Problem {
        let mut file = File::open("input/day6").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let res = buffer
            .trim_end()
            .split("\n\n")
            .map(|x| Group::new(x)).collect();

        Problem { data: res }
    }

    pub fn solve() {
        let problem = Problem::new();
        println!("Part a: {}", problem.part_a());
        println!("Part b: {}", problem.part_b());
    }

    fn part_a(&self) -> u32 {
        self.data.iter().map(|x| x.anyone_positive()).sum()
    }

    fn part_b(&self) -> u32 {
        self.data.iter().map(|x| x.everyone_positive()).sum()
    }
}
