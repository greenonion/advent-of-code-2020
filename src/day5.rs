use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

struct BoardingPass {
    ticket: String,
}

impl BoardingPass {
    pub fn new(ticket: &str) -> BoardingPass {
        BoardingPass {
            ticket: String::from(ticket)
        }
    }

    pub fn seat_id(&self) -> u32 {
        self.row() * 8 + self.column()
    }

    pub fn row(&self) -> u32 {
        let mut min = 0;
        let mut max = 128;

        for s in self.ticket[0..7].bytes() {
            let distance = max - min;
            if s == b'F' {
                max = max - distance / 2;
            } else if s == b'B' {
                min = min + distance / 2;
            }
        }

        min
    }

    pub fn column(&self) -> u32 {
        let mut min = 0;
        let mut max = 8;

        for s in self.ticket[7..10].bytes() {
            let distance = max - min;
            if s == b'L' {
                max = max - distance / 2;
            } else if s == b'R' {
                min = min + distance / 2;
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_ids() {
        let boarding_pass = BoardingPass::new("FBFBBFFRLR");
        assert_eq!(boarding_pass.row(), 44);
        assert_eq!(boarding_pass.column(), 5);
        assert_eq!(boarding_pass.seat_id(), 357);

        let boarding_pass = BoardingPass::new("BFFFBBFRRR");
        assert_eq!(boarding_pass.row(), 70);
        assert_eq!(boarding_pass.column(), 7);
        assert_eq!(boarding_pass.seat_id(), 567);

        let boarding_pass = BoardingPass::new("FFFBBBFRRR");
        assert_eq!(boarding_pass.row(), 14);
        assert_eq!(boarding_pass.column(), 7);
        assert_eq!(boarding_pass.seat_id(), 119);

        let boarding_pass = BoardingPass::new("BBFFBBFRLL");
        assert_eq!(boarding_pass.row(), 102);
        assert_eq!(boarding_pass.column(), 4);
        assert_eq!(boarding_pass.seat_id(), 820);
    }
}

pub struct Problem {
    data: Vec<BoardingPass>,
}

impl Problem {
    pub fn new() -> Problem {
        let mut file = File::open("input/day5").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let res = buffer
            .trim_end()
            .split("\n")
            .map(|x| BoardingPass::new(x)).collect();

        Problem { data: res }
    }

    pub fn solve() {
        let problem = Problem::new();
        println!("Part a: {}", problem.part_a());
        println!("Part b: {}", problem.part_b());
    }

    fn part_a(&self) -> u32 {
        self.data.iter().max_by_key(|bp| bp.seat_id()).unwrap().seat_id()
    }

    fn part_b(&self) -> u32 {
        let mut possible_seats: HashSet<u32> = HashSet::new();
        for i in 0..(128*8) {
            possible_seats.insert(i);
        }

        let mut seats_in_list: HashSet<u32> = HashSet::new();
        for bp in self.data.iter() {
            seats_in_list.insert(bp.seat_id());
        }

        let available_seats: HashSet<_>= possible_seats
            .difference(&seats_in_list)
            .collect();

        for seat in &available_seats {
            if seats_in_list.contains(&(*seat + 1)) &&
                seats_in_list.contains(&(*seat - 1)) {
                    return **seat;
                }
        }

        return 0;
    }

 }
