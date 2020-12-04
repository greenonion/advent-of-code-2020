use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct Problem {
    data: Vec<String>,
}

struct UserPassword {
    minimum: u32,
    maximum: u32,
    character: String,
    password: String,
}

impl FromStr for UserPassword {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(": ").collect();
        let policy = items[0];
        let password = items[1];
        let parts: Vec<&str> = policy.split(" ").collect();
        let limits: Vec<u32> = parts[0].split("-").map(|x| x.parse().unwrap()).collect();
        let character = parts[1];

        Ok(UserPassword {
            minimum: limits[0],
            maximum: limits[1],
            character: String::from(character),
            password: String::from(password),
        })
    }
}

impl UserPassword {
    fn is_valid(&self) -> bool {
        let occs: u32 = self.password.matches(&self.character).count() as u32;

        if occs >= self.minimum && occs <= self.maximum {
            return true;
        } else {
            return false;
        }
    }
}

struct UserPassword2 {
    pos1: usize,
    pos2: usize,
    character: String,
    password: String,
}

impl FromStr for UserPassword2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(": ").collect();
        let policy = items[0];
        let password = items[1];
        let parts: Vec<&str> = policy.split(" ").collect();
        let limits: Vec<u32> = parts[0].split("-").map(|x| x.parse().unwrap()).collect();
        let character = parts[1];
        let pos1 = (limits[0] - 1) as usize;
        let pos2 = (limits[1] - 1) as usize;

        Ok(UserPassword2 {
            pos1,
            pos2,
            character: String::from(character),
            password: String::from(password),
        })
    }
}

impl UserPassword2 {
    fn is_valid(&self) -> bool {
        let c1 = &self.password[self.pos1..self.pos1 + 1];
        let c2 = &self.password[self.pos2..self.pos2 + 1];

        if c1 == self.character && c2 == self.character {
            return false;
        } else if c1 == self.character || c2 == self.character {
            return true;
        } else {
            return false;
        }
    }
}

impl Problem {
    pub fn new() -> Problem {
        let mut file = File::open("input/day2").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        // TODO: how can we achieve the same without copying data? lifetimes
        let v: Vec<_> = buffer
            .trim_end()
            .split("\n")
            .map(|x| String::from(x))
            .collect();

        Problem { data: v }
    }

    pub fn solve() {
        let problem = Problem::new();
        println!("Part a: {}", problem.part_a());
        println!("Part b: {}", problem.part_b());
    }

    fn part_a(&self) -> u32 {
        self.data
            .iter()
            .map(|l| UserPassword::from_str(&l).unwrap())
            .filter(|up| up.is_valid())
            .count() as u32
    }

    fn part_b(&self) -> u32 {
        self.data
            .iter()
            .map(|l| UserPassword2::from_str(&l).unwrap())
            .filter(|up| up.is_valid())
            .count() as u32
    }
}
