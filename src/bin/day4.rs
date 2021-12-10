#![feature(io_read_to_string)]

use std::fs::File;
use std::io::read_to_string;
use std::str::FromStr;

fn main() {
    let mut f = File::open("day4.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();

    let bs: BingoSolver = s.parse().unwrap();
    let score = bs.first_winner();
    assert_eq!(score, 58374);
    println!("Part one: {}", score);

    let score = bs.last_winner();
    assert_eq!(score, 11377);
    println!("Part two: {}", score);
}

type Score = u32;

#[derive(Debug)]
struct BingoSolver {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl BingoSolver {
    fn first_winner(&self) -> Score {
        let mut read = vec![];
        for number in &self.numbers {
            read.push(*number);
            if let Some(score) = self.boards.iter().find_map(|b| b.winner(&read)) {
                return score;
            }
        }
        unreachable!()
    }

    fn last_winner(&self) -> Score {
        let mut read = self.numbers.clone();
        while let Some(last) = read.pop() {
            if let Some(board) = self.boards.iter().find(|b| b.winner(&read).is_none()) {
                read.push(last);
                return board.score(&read);
            }
        }
        unreachable!()
    }
}

impl FromStr for BingoSolver {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('\n').expect("Has no numbers");
        let boards = right
            .split("\n\n")
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        let numbers = left
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self { numbers, boards })
    }
}

#[derive(Debug)]
struct Board {
    b: Vec<Vec<u8>>,
}

impl Board {
    fn winner(&self, read: &[u8]) -> Option<Score> {
        if self.row_win(read) || self.col_win(read) {
            return Some(self.score(read));
        }
        None
    }

    fn score(&self, read: &[u8]) -> Score {
        self.sum_unmarked(read) * *read.last().unwrap() as u32
    }

    fn sum_unmarked(&self, read: &[u8]) -> Score {
        self.rows()
            .flatten()
            .filter_map(|v| {
                if read.contains(v) {
                    return None;
                }
                Some(*v as u32)
            })
            .sum()
    }

    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.b.iter().map(|s| s.as_slice())
    }

    fn cols(&self) -> impl Iterator<Item = Vec<u8>> + '_ {
        (0..self.b[0].len()).map(|i| self.b.iter().map(|inner| inner[i]).collect())
    }

    fn row_win(&self, read: &[u8]) -> bool {
        self.rows().any(|row| row.iter().all(|v| read.contains(v)))
    }

    fn col_win(&self, read: &[u8]) -> bool {
        self.cols().any(|col| col.iter().all(|v| read.contains(v)))
    }
}

impl FromStr for Board {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<u8> = s
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            b: v.chunks(5).map(|c| c.to_vec()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = include_str!("day4.test.txt");
        let bs: BingoSolver = input.parse().unwrap();

        assert_eq!(bs.first_winner(), 4512);
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("day4.test.txt");
        let bs: BingoSolver = input.parse().unwrap();

        assert_eq!(bs.last_winner(), 1924);
    }

    #[test]
    fn load_board_works() {
        let input = include_str!("day4.test.txt");
        let bs: BingoSolver = input.parse().unwrap();

        assert_eq!(bs.numbers[0], 7);
        assert_eq!(*bs.numbers.last().unwrap(), 1);
        assert_eq!(bs.boards[0].b[0][0], 22);
        assert_eq!(bs.boards[0].b[0][1], 13);
        assert_eq!(bs.boards[0].b[1][0], 8);

        assert_eq!(bs.boards[1].b[0][0], 3);
    }

    #[test]
    fn row_win_works() {
        let board = Board {
            b: vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25],
            ],
        };

        assert!(board.row_win(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn col_win_works() {
        let board = Board {
            b: vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25],
            ],
        };
        assert!(board.col_win(&vec![1, 6, 11, 16, 21]));
    }
}
