use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    data: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let k: Vec<&str> = s.split(|c| c == ' ' || c == '\n').
            collect();
        let mut passport_h = HashMap::new();

        for field_value in k {
            let kv: Vec<&str> = field_value.split(":").collect();
            passport_h.insert(String::from(kv[0]),
                              String::from(kv[1]));
        }

        Ok(Passport{data: passport_h})
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        let mut required_fields = HashSet::new();
        for key in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned() {
            required_fields.insert(String::from(key));
        }

        let mut provided_fields = HashSet::new();
        for key in self.data.keys() {
            provided_fields.insert(String::from(key));
        }

        provided_fields.is_superset(&required_fields)
    }

    fn is_valid_strict(&self) -> bool {
        for (k, v) in self.data.iter() {
            match &k[..] {
                "byr" => {
                    if !Passport::is_valid_byr(v) {
                        return false
                    }
                },
                "iyr" => {
                    if !Passport::is_valid_iyr(v) {
                        return false
                    }
                },
                "eyr" => {
                    if !Passport::is_valid_eyr(v) {
                        return false
                    }
                },
                "hgt" => {
                    if !Passport::is_valid_hgt(v) {
                        return false
                    }
                },
                "hcl" => {
                    if !Passport::is_valid_hcl(v) {
                        return false
                    }
                },
                "ecl" => {
                    if !Passport::is_valid_ecl(v) {
                        return false
                    }
                },
                "pid" => {
                    if !Passport::is_valid_pid(v) {
                        return false
                    }
                }
                _ => ()
            }
        }
        return true
    }

    fn is_valid_byr(v: &str) -> bool {
        let re = Regex::new(r"^(\d{4})$").unwrap();
        for cap in re.captures_iter(v) {
            let byr: u32 = cap[1].parse().unwrap();
            return byr >= 1920 && byr <= 2002
        }

        return false
    }

    fn is_valid_iyr(v: &str) -> bool {
        let re = Regex::new(r"^(\d{4})$").unwrap();
        for cap in re.captures_iter(v) {
            let iyr: u32 = cap[1].parse().unwrap();
            return iyr >= 2010 && iyr <= 2020
        }

        return false
    }

    fn is_valid_eyr(v: &str) -> bool {
        let re = Regex::new(r"^(\d{4})$").unwrap();
        for cap in re.captures_iter(v) {
            let eyr: u32 = cap[1].parse().unwrap();
            return eyr >= 2020 && eyr <= 2030
        }

        return false
    }

    fn is_valid_hgt(v: &str) -> bool {
        let re = Regex::new(r"^(\d+)(in|cm)$").unwrap();

        for cap in re.captures_iter(v) {
            let height: u32 = cap[1].parse().unwrap();
            if &cap[2] == "cm" {
                return height >= 150 && height <= 193
            } else if &cap[2] == "in" {
                return height >= 59 && height <= 76
            }
        }

        false
    }

    fn is_valid_hcl(v: &str) -> bool {
        let re = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
        re.is_match(v)
    }

    fn is_valid_ecl(v: &str) -> bool {
        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        re.is_match(v)
    }

    fn is_valid_pid(v: &str) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();
        re.is_match(v)
    }
}

pub struct Problem {
    data: Vec<Passport>,
}

impl Problem {
    pub fn new() -> Problem {
        let mut file = File::open("input/day4").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let res = buffer
            .trim_end()
            .split("\n\n")
            .map(|x| x.parse::<Passport>().unwrap()).collect();

        Problem { data: res }
    }

    pub fn solve() {
        let problem = Problem::new();
        println!("Part a: {}", problem.part_a());
        println!("Part b: {}", problem.part_b());
    }

    fn part_a(&self) -> u32 {
        self.data.iter().filter(|x| x.is_valid()).count() as u32
    }

    fn part_b(&self) -> u32 {
        self.data.iter().filter(|x| x.is_valid() && x.is_valid_strict()).count() as u32
    }
}
