#![feature(io_read_to_string)]
use std::fs::File;
use std::io::read_to_string;

fn main() {
    let mut f = File::open("day9.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();
    let hm: HeightMap = s.parse().unwrap();

    let risk = hm.risk_factor();
    assert_eq!(risk, 514);
    println!("Part one: {}", risk);

    let score = hm.basin_score();
    assert_eq!(score, 1103130);
    println!("Part two: {}", score);
}

struct HeightMap {
    hm: Vec<Vec<u32>>,
}

impl HeightMap {
    fn risk_factor(&self) -> u32 {
        let mut risk = 0;
        for row in 0..self.hm.len() {
            for col in 0..self.hm[0].len() {
                if self.is_low_point(row, col) {
                    risk += self.hm[row][col] + 1;
                }
            }
        }
        risk
    }

    fn basin_score(&self) -> u32 {
        // collect low points
        let mut low_points = vec![];
        for row in 0..self.hm.len() {
            for col in 0..self.hm[0].len() {
                if self.is_low_point(row, col) {
                    low_points.push((row, col))
                }
            }
        }

        // map low points to basin sizes
        let mut scores: Vec<u32> = low_points
            .into_iter()
            .map(|p| {
                let mut visited = vec![];
                self.basin_size(p.0, p.1, &mut visited)
            })
            .collect();

        // return product of top 3 scores
        scores.sort_unstable();
        scores
            .into_iter()
            .rev()
            .take(3)
            .reduce(std::ops::Mul::mul)
            .unwrap()
    }

    fn basin_size(&self, row: usize, col: usize, inspected: &mut Vec<(usize, usize)>) -> u32 {
        if inspected.contains(&(row, col)) {
            return 0;
        }
        inspected.push((row, col));
        if self.hm[row][col] == 9 {
            return 0;
        }

        let mut sum = 1;
        for point in self.surrounding_indicies(row, col) {
            sum += self.basin_size(point.0, point.1, inspected);
        }
        sum
    }

    fn is_low_point(&self, row: usize, col: usize) -> bool {
        self.surrounding_points(row, col)
            .into_iter()
            .all(|v| self.hm[row][col] < v)
    }

    fn surrounding_indicies(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        // push upper
        if row > 0 {
            v.push((row - 1, col));
        }
        // push lower
        if row < self.hm.len() - 1 {
            v.push((row + 1, col));
        }

        // push left
        if col > 0 {
            v.push((row, col - 1));
        }
        // push right
        if col < self.hm[0].len() - 1 {
            v.push((row, col + 1))
        }
        v
    }

    fn surrounding_points(&self, row: usize, col: usize) -> Vec<u32> {
        self.surrounding_indicies(row, col)
            .into_iter()
            .map(|(i, j)| self.hm[i][j])
            .collect()
    }
}

impl std::str::FromStr for HeightMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hm = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Could not parse digit"))
                    .collect()
            })
            .collect();
        Ok(Self { hm })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_works() {
        let s = include_str!("day9.test.txt");
        let hm: HeightMap = s.parse().unwrap();
        let score = hm.basin_score();
        assert_eq!(score, 1134);
    }

    #[test]
    fn simple_part_two_works() {
        let s = r#"999
919
999"#;
        let hm: HeightMap = s.parse().unwrap();
        let mut visited = vec![];
        let size = hm.basin_size(1, 1, &mut visited);
        dbg!(hm.hm[1][1]);
        assert_eq!(size, 1);
    }

    #[test]
    fn part_one_works() {
        let s = include_str!("day9.test.txt");
        let hm: HeightMap = s.parse().unwrap();
        let risk = hm.risk_factor();
        assert_eq!(risk, 15);
    }
}
