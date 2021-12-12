#![feature(io_read_to_string)]
use std::fs::File;
use std::io::read_to_string;

fn main() {
    let mut f = File::open("day11.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();
    let mut m: EnergyMap = s.parse().unwrap();
    let flashes = m.flashes_after(100);
    println!("Part one: {}", flashes);
    assert_eq!(flashes, 1679);

    let mut m: EnergyMap = s.parse().unwrap();
    let steps = m.steps_to_simul();
    println!("Part two: {}", steps);
    assert_eq!(steps, 519);
}

#[derive(Debug, Eq, PartialEq)]
struct EnergyMap {
    map: Vec<Vec<u8>>,
}

impl EnergyMap {
    fn flashes_after(&mut self, steps: usize) -> u64 {
        (0..steps).fold(0, |acc, _| acc + self.step())
    }

    fn steps_to_simul(&mut self) -> u64 {
        let mut steps = 0;
        while !self.map.iter().flatten().all(|v| *v == 0) {
            self.step();
            steps += 1;
        }
        steps
    }

    fn step(&mut self) -> u64 {
        // increment all levels by 1
        let mut flashed = vec![];
        let mut num_flashed = 0;
        for (row, inner) in self.map.iter_mut().enumerate() {
            for (col, value) in inner.iter_mut().enumerate() {
                *value += 1;
                if *value > 9 {
                    flashed.push((row, col));
                }
            }
        }

        // octos w/ levels greater than 9 flash and cause adjectent (and diagonal) levels to increment
        let mut visited = flashed.clone();
        for (row, col) in flashed.into_iter() {
            num_flashed += 1;
            num_flashed += self.flash(row, col, &mut visited);
        }

        // any octo that flashed is set to 0
        for inner in self.map.iter_mut() {
            for v in inner.iter_mut() {
                if *v > 9 {
                    *v = 0;
                }
            }
        }
        num_flashed
    }

    fn flash(&mut self, row: usize, col: usize, flashed: &mut Vec<(usize, usize)>) -> u64 {
        flashed.push((row, col));
        let adjacent = self.get_adjacent(row, col);
        let mut num_flashed = 0;

        for (row, col) in adjacent {
            let value = &mut self.map[row][col];
            *value += 1;
            if !flashed.contains(&(row, col)) {
                if *value > 9 {
                    num_flashed += 1;
                    num_flashed += self.flash(row, col, flashed);
                }
            }
        }
        num_flashed
    }

    fn get_adjacent(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        let height = self.map.len();
        let width = self.map[0].len();
        // push upper left
        if row > 0 && col > 0 {
            v.push((row - 1, col - 1))
        }

        // push upper right
        if row > 0 && col < width - 1 {
            v.push((row - 1, col + 1))
        }

        // push lower left
        if row < height - 1 && col > 0 {
            v.push((row + 1, col - 1))
        }

        // push lower right
        if row < height - 1 && col < width - 1 {
            v.push((row + 1, col + 1))
        }

        // push upper
        if row > 0 {
            v.push((row - 1, col));
        }
        // push lower
        if row < height - 1 {
            v.push((row + 1, col));
        }

        // push left
        if col > 0 {
            v.push((row, col - 1));
        }
        // push right
        if col < width - 1 {
            v.push((row, col + 1))
        }
        v
    }

    #[allow(dead_code)]
    fn print(&self) {
        for inner in self.map.iter() {
            for v in inner.iter() {
                print!("{}", v);
            }
            println!();
        }
        println!("=================================")
    }
}

impl std::str::FromStr for EnergyMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Ok(Self { map })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn first_simul_flash_works() {
        let s = include_str!("day11.test.txt");
        let mut m: EnergyMap = s.parse().unwrap();
        let ans = m.steps_to_simul();
        assert_eq!(ans, 195);
    }
    #[test]
    fn part_one_works() {
        let s = include_str!("day11.test.txt");
        let mut m: EnergyMap = s.parse().unwrap();
        let flashes = m.flashes_after(100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn part_one_simple() {
        let s = r#"11111
19991
19191
19991
11111"#;

        let mut m: EnergyMap = s.parse().unwrap();
        let changed = m.step();

        let expected: EnergyMap = r#"34543
40004
50005
40004
34543"#
            .parse()
            .unwrap();

        assert_eq!(m, expected);
        assert_eq!(changed, 9);

        let expected: EnergyMap = r#"45654
51115
61116
51115
45654"#
            .parse()
            .unwrap();
        m.step();
        assert_eq!(m, expected);
    }
}
