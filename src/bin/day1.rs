#![feature(io_read_to_string)]

use std::fs::File;
use std::io::read_to_string;

fn main() {
    let mut f = File::open("day1.1.txt").unwrap();
    let is = read_to_string(&mut f).unwrap();
    let input: Vec<i32> = is
        .split_whitespace()
        .map(|ns| ns.parse().unwrap())
        .collect();
    let answer = count_increases_2(&input);
    assert_eq!(answer, 1195);
    println!("Part 1 answer: {}", answer);

    let answer = count_increases_3(&input);
    assert_eq!(answer, 1235);
    println!("Part 2 answer: {}", answer);
}

fn count_increases_2(input: &[i32]) -> i32 {
    input
        .windows(2)
        .map(|slice| {
            if slice[0] < slice[1] {
                return 1;
            }
            0
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

fn count_increases_3(input: &[i32]) -> i32 {
    input
        .windows(3)
        .map(|slice| slice.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|slice| {
            if slice[0] < slice[1] {
                return 1;
            }
            0
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[test]
fn small() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_increases_2(&input), 7);
}

#[test]
fn window_3() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_increases_3(&input), 5);
}
