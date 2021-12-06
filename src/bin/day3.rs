#![feature(io_read_to_string)]
use std::collections::HashMap;
use std::fs::File;
use std::io::read_to_string;

fn main() {
    let mut f = File::open("day3.1.txt").unwrap();
    let s = read_to_string(&mut f).unwrap();
    let s = s.split_whitespace().collect::<Vec<_>>();

    let (g, e) = calc_part_one(&s);
    assert_eq!(g * e, 3_687_446);
    println!("Answer Part 1: {}", g * e);
}

fn calc_part_one(input: &[&str]) -> (u64, u64) {
    input
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<HashMap<char, usize>>, s| {
            for (i, c) in s.chars().enumerate() {
                if let Some(map) = acc.get_mut(i) {
                    *map.entry(c).or_default() += 1;
                } else {
                    let mut map = HashMap::new();
                    *map.entry(c).or_default() += 1;
                    acc.push(map);
                }
            }
            acc
        })
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, map)| {
            let zeros = map.get(&'0').cloned().unwrap_or_default();
            let ones = map.get(&'1').cloned().unwrap_or_default();
            let gamma = if ones > zeros { 1 } else { 0 };
            let epsilon = if ones < zeros { 1 } else { 0 };
            (gamma, epsilon, i)
        })
        .fold(
            Default::default(),
            |(g_acc, e_acc), (gamma, epsilon, power)| {
                let ng = g_acc + gamma * 2u64.pow(power as u32);
                let ne = e_acc + epsilon * 2u64.pow(power as u32);
                (ng, ne)
            },
        )
}

fn calc_freq_filter<F: Fn(usize, usize) -> bool>() {}
fn calc_part_two(input: &[&str]) -> (u64, u64) {
    let mut freq_maps: Vec<HashMap<char, usize>> = Vec::new();
    let str_len = input[0].len();
    let mut skipping = vec![];
    for col in 0..str_len {
        for (i, s) in input.iter().enumerate() {
            if !skipping.contains(&i) {
                let chars = s.chars().collect::<Vec<_>>();
                if let Some(map) = freq_maps.get_mut(col) {
                    *map.entry(chars[col]).or_default() += 1;
                } else {
                    let mut map = HashMap::new();
                    *map.entry(chars[col]).or_default() += 1;
                    freq_maps.push(map);
                }
            }
        }

        // o2 keeep w/ most common
        // co2 keep w/ least common
        let zeros = freq_maps[col].get(&'0').cloned().unwrap_or_default();
        let ones = freq_maps[col].get(&'1').cloned().unwrap_or_default();
        if ones >= zeros {
            //keep ones in position col
            for (i, s) in input.iter().enumerate() {
                let chars = s.chars().collect::<Vec<_>>();
                if chars[col] == '0' && !skipping.contains(&i) {
                    skipping.push(i)
                }
            }
        } else {
            // keep zeroes in position col
            for (i, s) in input.iter().enumerate() {
                let chars = s.chars().collect::<Vec<_>>();
                if chars[col] == '1' && !skipping.contains(&i) {
                    skipping.push(i)
                }
            }
        }

        println!("Iteration: {}", col);
        for i in &skipping {
            dbg!(input[*i]);
        }
    }

    dbg!(freq_maps);
    panic!();

    freq_maps
        .into_iter()
        //.inspect(|v| {dbg!(&v);})
        .rev()
        .enumerate()
        .map(|(i, map)| {
            let zeros = map.get(&'0').cloned().unwrap_or_default();
            let ones = map.get(&'1').cloned().unwrap_or_default();
            let gamma = if ones > zeros { 1 } else { 0 };
            let epsilon = if ones < zeros { 1 } else { 0 };
            (gamma, epsilon, i)
        })
        .fold(
            Default::default(),
            |(g_acc, e_acc), (gamma, epsilon, power)| {
                let ng = g_acc + gamma * 2u64.pow(power as u32);
                let ne = e_acc + epsilon * 2u64.pow(power as u32);
                (ng, ne)
            },
        )
}
#[test]
fn part_one_works() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    let (g, e) = calc_part_one(&input);
    assert_eq!(g * e, 198);
}

#[test]
#[ignore]
fn part_two_works() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let (o, co) = calc_part_two(&input);
    assert_eq!(o * co, 230)
}
