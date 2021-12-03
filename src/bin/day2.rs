#![feature(io_read_to_string)]
use std::fs::File;
use std::io::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut f = File::open("day2.1.txt").unwrap();
    let data = read_to_string(&mut f).unwrap();
    // Not sure why but there is an empty string at the end after the split operation
    let input = data.split('\n').collect::<Vec<_>>();
    let (x, y) = final_position(&input[..input.len() - 1]);
    assert_eq!(x * y, 1840243);
    println!("Final Position: ({}, {}) = {}", x, y, x * y);

    let mut f = File::open("day2.2.txt").unwrap();
    let data = read_to_string(&mut f).unwrap();
    let input = data.split('\n').collect::<Vec<_>>();

    // Not sure why but there is an empty string at the end after the split operation
    let (x, y) = final_position_2(&input[..input.len() - 1]);
    assert_eq!(x * y, 1727785422);
    println!("Final Position 2: ({}, {}) = {}", x, y, x * y);
}

enum Command {
    Forward(i32),
    Back(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split_whitespace().collect();
        match v[0].to_ascii_lowercase().as_ref() {
            "forward" => Ok(Self::Forward(v[1].parse()?)),
            "backward" => Ok(Self::Back(v[1].parse()?)),
            "up" => Ok(Self::Up(v[1].parse()?)),
            "down" => Ok(Self::Down(v[1].parse()?)),
            _ => unreachable!(),
        }
    }
}

fn final_position(input: &[&str]) -> (i32, i32) {
    input.iter().map(|s| s.parse::<Command>().unwrap()).fold(
        Default::default(),
        |(x, y), command| match command {
            Command::Forward(v) => (x + v, y),
            Command::Back(v) => (x - v, y),
            Command::Up(v) => (x, y - v),
            Command::Down(v) => (x, y + v),
        },
    )
}

fn final_position_2(input: &[&str]) -> (i32, i32) {
    let out: (i32, i32, i32) = input.iter().map(|s| s.parse::<Command>().unwrap()).fold(
        Default::default(),
        |(x, y, aim), command| match command {
            Command::Forward(v) => (x + v, y + aim * v, aim),
            Command::Up(v) => (x, y, aim - v),
            Command::Down(v) => (x, y, aim + v),
            Command::Back(_) => unreachable!(),
        },
    );

    (out.0, out.1)
}

#[test]
fn part_one() {
    let input = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let (x, y) = final_position(&input);
    assert_eq!(x, 15);
    assert_eq!(y, 10);
    assert_eq!(x * y, 150);
}

#[test]
fn part_two() {
    let input = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let (x, y) = final_position_2(&input);
    assert_eq!(x, 15);
    assert_eq!(y, 60);
    assert_eq!(x * y, 900);
}
