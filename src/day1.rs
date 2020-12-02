use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Problem {
    data: Vec<u32>,
}

impl Problem {
    pub fn new() -> Problem {
        let mut file = File::open("input/day1").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let v: Vec<u32> = buffer
            .trim_end()
            .split("\n")
            .map(|x| x.parse().unwrap())
            .collect();

        Problem { data: v }
    }

    pub fn solve() {
        let problem = Problem::new();
        problem.part_a();
        problem.part_b();
    }

    fn part_a(&self) {
        for i in 0..(self.data.len() - 1) {
            for j in 1..self.data.len() {
                let a = self.data[i];
                let b = self.data[j];
                if a + b == 2020 {
                    println!("{}", a * b);
                    return;
                }
            }
        }
    }

    fn part_b(&self) {
        for i in 0..(self.data.len() - 2) {
            for j in 1..(self.data.len() - 1) {
                for k in 1..self.data.len() {
                    let a = self.data[i];
                    let b = self.data[j];
                    let c = self.data[k];

                    if a + b + c == 2020 {
                        println!("{}", a * b * c);
                        return;
                    }
                }
            }
        }
    }
}
