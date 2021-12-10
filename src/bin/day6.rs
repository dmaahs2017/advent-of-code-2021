#![feature(io_read_to_string)]
use std::fs::File;
use std::io::read_to_string;
use std::str::FromStr;

fn main() {
    let mut f = File::open("day6.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();
    let input = &s[..s.len() - 1];

    let mut school: School = input.parse().unwrap();
    school.pass_time(80);
    println!("Part one: {}", school.total());
    assert_eq!(school.total(), 372984);

    let mut school: School = input.parse().unwrap();
    school.pass_time(256);
    println!("Part two: {}", school.total());
    assert_eq!(school.total(), 1681503251694);
}

/// School of lantern fish
#[derive(Debug)]
struct School {
    v: Vec<u64>,
}

impl FromStr for School {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            v: s.split(',')
                .map(|s| s.parse::<u8>())
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .fold(vec![0; 9], |mut acc, value| {
                    acc[*value as usize] += 1;
                    acc
                }),
        })
    }
}

impl School {
    fn increment_day(&mut self) {
        self.v[7] += self.v[0];
        let mut iter = self.v.iter_mut().rev();
        let prev = iter.next().unwrap();
        for v in iter {
            std::mem::swap(v, prev);
        }
        self.v[8] = *prev;
    }

    fn print(&self) {
        println!(
            "{}",
            self.v
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
    }

    fn pass_time(&mut self, days: usize) -> u64 {
        for _ in 0..days {
            self.increment_day();
        }
        self.total()
    }

    fn total(&self) -> u64 {
        self.v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_works() {
        let input = "1,2,3,4";
        let s = School::from_str(input).unwrap();
        assert_eq!(s.v, vec![0, 1, 1, 1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn incr_day_works() {
        let mut s = School::from_str("0,1,2,3,4,5,6,7,8").unwrap();
        assert_eq!(s.v, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
        s.increment_day();
        assert_eq!(s.v, vec![1, 1, 1, 1, 1, 1, 2, 1, 1]);
    }

    #[test]
    fn pass_18() {
        let input = "3,4,3,1,2";
        let mut s = School::from_str(input).unwrap();
        let a = s.pass_time(18);
        assert_eq!(a, 26);
    }

    #[test]
    fn pass_80() {
        let input = "3,4,3,1,2";
        let mut s = School::from_str(input).unwrap();
        let b = s.pass_time(80);
        assert_eq!(b, 5934)
    }
}
