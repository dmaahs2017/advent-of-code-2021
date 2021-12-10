#![feature(io_read_to_string)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::read_to_string;

fn main() {
    let mut f = File::open("day10.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();
    let score = syntax_error_score(&s, &ERR_POINT_TABLE);
    assert_eq!(score, 367059);
    println!("Part one: {}", score);

    let score = completion_score(&s, &COMPLETION_POINT_TABLE);
    assert_eq!(score, 1952146692);
    println!("Part two: {}", score);
}

lazy_static! {
    static ref ERR_POINT_TABLE: HashMap<char, u64> = {
        let mut map = HashMap::new();
        map.insert(')', 3);
        map.insert(']', 57);
        map.insert('}', 1197);
        map.insert('>', 25137);
        map
    };
}

lazy_static! {
    static ref COMPLETION_POINT_TABLE: HashMap<char, u64> = {
        let mut map = HashMap::new();
        map.insert(')', 1);
        map.insert(']', 2);
        map.insert('}', 3);
        map.insert('>', 4);
        map
    };
}

const OPENING_CHARS: [char; 4] = ['(', '[', '{', '<'];
const CLOSING_CHARS: [char; 4] = [')', ']', '}', '>'];

lazy_static! {
    static ref MATCH: HashMap<char, char> = {
        let mut map = HashMap::new();
        for (a, b) in OPENING_CHARS.into_iter().zip(CLOSING_CHARS.into_iter()) {
            map.insert(a, b);
        }

        map
    };
}

fn completion_score(s: &str, point_table: &HashMap<char, u64>) -> u64 {
    let mut v: Vec<u64> = s
        .lines()
        .filter_map(|line| {
            let mut stack = vec![];
            for c in line.chars() {
                if OPENING_CHARS.contains(&c) {
                    stack.push(c);
                    continue;
                }
                // Otherwise closing character must match the last character
                if let Some(&opening) = stack.last() {
                    // We have matching characters.
                    if *MATCH.get(&opening).expect("Match should exits") == c {
                        stack.pop();
                    } else {
                        // Mis-matching characters
                        return None;
                    }
                } else {
                    // Close character with no open character
                    return None;
                }
            }

            let score = stack
                .into_iter()
                .map(|c| *MATCH.get(&c).expect("Matching character DNE"))
                .rfold(0, |score, c| {
                    score * 5
                        + point_table
                            .get(&c)
                            .expect("Point table has no value for the character")
                });
            Some(score)
        })
        .collect();

    v.sort_unstable();
    v[v.len() / 2]
}

fn syntax_error_score(s: &str, point_table: &HashMap<char, u64>) -> u64 {
    s.lines().fold(0, |score, line| {
        let mut stack = vec![];
        for c in line.chars() {
            if OPENING_CHARS.contains(&c) {
                stack.push(c);
                continue;
            }

            // Otherwise closing character must match the last character
            if let Some(&opening) = stack.last() {
                // We have matching characters.
                if *MATCH.get(&opening).expect("Match should exits") == c {
                    stack.pop();
                } else {
                    // Mis-matching characters
                    return score + point_table.get(&c).expect("Character not in point table");
                }
            } else {
                // Close character with no open character
                return score + point_table.get(&c).expect("Character not in point table");
            }
        }
        score
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_works() {
        let s = include_str!("day10.test.txt");
        let score = completion_score(s, &COMPLETION_POINT_TABLE);
        assert_eq!(score, 288957)
    }

    #[test]
    fn part_two_simple_works() {
        let s = "(";
        let score = completion_score(s, &COMPLETION_POINT_TABLE);
        assert_eq!(score, 1);

        let s = "({";
        let score = completion_score(s, &COMPLETION_POINT_TABLE);
        assert_eq!(score, 16);
    }

    #[test]
    fn part_one_works() {
        let s = include_str!("day10.test.txt");
        let score = syntax_error_score(s, &ERR_POINT_TABLE);
        assert_eq!(score, 26397)
    }

    #[test]
    fn simple_works() {
        let s = ")";
        let score = syntax_error_score(s, &ERR_POINT_TABLE);
        assert_eq!(score, 3)
    }
}
