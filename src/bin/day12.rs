#![feature(io_read_to_string)]
use std::fs::File;
use std::io::read_to_string;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut f = File::open("day12.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();

    let cm: CaveMap = s.parse().unwrap();
    let x = cm.num_paths();
    println!("Part one: {}", x);
}

#[derive(Debug)]
struct CaveMap {
    connections: Vec<(Cave, Cave)>,
}

impl CaveMap {
    fn num_paths(&self) -> u64 {
        let paths: Vec<Vec<Cave>> = self.enumerate_paths();
        paths.len() as u64
    }

    fn enumerate_paths(&self) -> Vec<Vec<Cave>> {
        let mut paths = vec![];

        while let Some(path) = self.next_path(&paths) {
            paths.push(path);
        }

        paths
    }

    fn next_path(&self, paths: &Vec<Vec<Cave>>) -> Option<Vec<Cave>> {
        let start = Cave::Small("start".to_string());
        let mut visited = vec![start];
        let mut attempts = 0;

        while *visited.last().unwrap() != Cave::Small("end".to_string()) {
            let mut next_turns = self.available_turns(visited.last().unwrap());
            let mut rng = thread_rng();
            next_turns.shuffle(&mut rng);
            for cave in next_turns {
                if cave.is_small() && !visited.contains(cave) {
                    visited.push(cave.clone());
                    if paths.contains(&visited) {
                        visited.pop();
                    }
                } else {
                    visited.push(cave.clone());
                    if paths.contains(&visited) {
                        visited.pop();
                    }
                }
            }

            attempts += 1;
            if attempts == 2000 {
                return None
            }
        }
        for v in &visited {
            print!("{}, ",v.text());
        }
        println!();

        Some(visited)
    }

    fn available_turns(&self, current_cave: &Cave) -> Vec<&Cave> {
        self.connections.iter().filter_map(|p| {
            if p.0 == *current_cave {
                return Some(&p.1)
            }
            if p.1 == *current_cave {
                return Some(&p.0)
            }
            None
        }).collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Cave {
    Large(String),
    Small(String),
}

impl Cave {
    fn is_small(&self) -> bool {
        if let Cave::Small(_) = self {
            return true;
        }
        false
    }

    fn is_large(&self) -> bool {
        !self.is_small()
    }

    fn text(&self) -> &str {
        match self {
            Cave::Large(s) => &s,
            Cave::Small(s) => &s,
        }
    }
}

impl std::str::FromStr for CaveMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connections = s
            .lines()
            .map(|line| {
                let (first, second) = line.split_once('-').unwrap();
                let c1 = if first.chars().all(char::is_lowercase) {
                    Cave::Small(first.to_string())
                } else {
                    Cave::Large(first.to_string())
                };

                let c2 = if second.chars().all(char::is_lowercase) {
                    Cave::Small(second.to_string())
                } else {
                    Cave::Large(second.to_string())
                };

                (c1, c2)
            })
            .collect();

        Ok(Self { connections })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let s = include_str!("day12.test1.txt");
        let cm: CaveMap = s.parse().unwrap();
        let paths = cm.num_paths();
        assert_eq!(paths, 10);
    }

    #[test]
    fn it_works_2() {
        let s = include_str!("day12.test2.txt");
        let cm: CaveMap = s.parse().unwrap();
        let paths = cm.num_paths();
        assert_eq!(paths, 10);
    }

    #[test]
    fn it_works3() {
        let s = include_str!("day12.test3.txt");
        let cm: CaveMap = s.parse().unwrap();
        let paths = cm.num_paths();
        assert_eq!(paths, 10);
    }

    #[test]
    fn available_paths_works() {
        let s = include_str!("day12.test1.txt");
        let cm: CaveMap = s.parse().unwrap();
        let paths = cm.available_turns(&Cave::Small("start".to_string()));
        assert_eq!(paths, vec![
            &Cave::Large("A".to_string()),
            &Cave::Small("b".to_string()),
        ]);

        let paths = cm.available_turns(&Cave::Small("b".to_string()));
        assert_eq!(paths, vec![
            &Cave::Small("start".to_string()),
            &Cave::Large("A".to_string()),
            &Cave::Small("d".to_string()),
            &Cave::Small("end".to_string()),
        ]);
    }
}
