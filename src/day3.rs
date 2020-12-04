use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Down,
    Right,
}

#[derive(PartialEq, Debug)]
enum SquareKind {
    Open,
    Tree,
}

impl FromStr for SquareKind {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "." {
            return Ok(SquareKind::Open);
        } else if s == "#" {
            return Ok(SquareKind::Tree);
        }

        Ok(SquareKind::Open)
    }
}

#[derive(Debug)]
struct Level {
    squares: Vec<SquareKind>,
}

impl FromStr for Level {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let squares = s
            .chars()
            .map(|c| c.to_string().parse::<SquareKind>().unwrap())
            .collect();
        Ok(Level { squares: squares })
    }
}

pub struct Problem {
    data: Vec<Level>,
    position: Point,
}

impl Problem {
    pub fn new() -> Problem {
        let mut file = File::open("input/day3").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let v: Vec<Level> = buffer
            .trim_end()
            .split("\n")
            .map(|x| x.parse::<Level>().unwrap())
            .collect();

        Problem {
            data: v,
            position: Point { x: 0, y: 0 },
        }
    }

    pub fn solve() {
        let mut map = Problem::new();
        println!("Part a: {}", map.part_a());
        println!("Part b: {}", map.part_b());
    }

    fn part_a(&mut self) -> u64 {
        self.trees_at_slope(3, 1)
    }

    fn part_b(&mut self) -> u64 {
        self.trees_at_slope(1, 1)
            * self.trees_at_slope(3, 1)
            * self.trees_at_slope(5, 1)
            * self.trees_at_slope(7, 1)
            * self.trees_at_slope(1, 2)
    }

    fn trees_at_slope(&mut self, right: usize, down: usize) -> u64 {
        self.position = Point { x: 0, y: 0 };
        let mut trees = 0;

        while !self.is_at_bottom() {
            for _ in 0..right {
                self.move_to(Direction::Right);
            }
            for _ in 0..down {
                self.move_to(Direction::Down);
            }

            if *self.square() == SquareKind::Tree {
                trees += 1;
            }
        }

        trees
    }

    fn square_kind_at(&self, pos: &Point) -> &SquareKind {
        &self.data[pos.x].squares[pos.y]
    }

    fn square(&self) -> &SquareKind {
        self.square_kind_at(&self.position)
    }

    fn move_to(&mut self, direction: Direction) -> &SquareKind {
        match direction {
            Direction::Right => self.move_right(),
            Direction::Down => self.move_down(),
        }
    }

    fn move_right(&mut self) -> &SquareKind {
        let length = self.data[self.position.y].squares.len();
        self.position.y = (self.position.y + 1) % length;
        self.square()
    }

    fn move_down(&mut self) -> &SquareKind {
        if !self.is_at_bottom() {
            self.position.x += 1;
        }
        self.square()
    }

    fn is_at_bottom(&self) -> bool {
        self.position.x == self.data.len() - 1
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
