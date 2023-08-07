use std::{cmp::Ordering, collections::HashMap};

use lazy_static::lazy_static;

use regex::Regex;

const KIND: &str = "children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes";

lazy_static! {
    static ref RE: Regex = Regex::new(&format!(
        r"Sue \d+: ({KIND}): (\d+)(?:(?:, )?({KIND}): (\d+))(?:(?:, )?({KIND}): (\d+))"
    ))
    .unwrap();
}

fn get_idx(hashmap: HashMap<&str, (Ordering, usize)>, input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            RE.captures(line)
                .map(|capture| {
                    let (_, data): (_, [&str; 6]) = capture.extract();
                    data.chunks(2)
                        .map(|chunk| (chunk[0], chunk[1].parse::<usize>().unwrap()))
                        .collect::<HashMap<_, _>>()
                })
                .unwrap()
        })
        .enumerate()
        .find(|(_, message)| {
            message.iter().all(|(key, val_1)| {
                let (ord, val_2) = hashmap.get(key).unwrap();
                val_1.cmp(val_2) == *ord
            })
        })
        .map(|(idx, _)| (idx + 1) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let hashmap: HashMap<&'static str, (Ordering, usize)> = {
        let mut m = HashMap::new();
        m.insert("children", (Ordering::Equal, 3));
        m.insert("cats", (Ordering::Equal, 7));
        m.insert("samoyeds", (Ordering::Equal, 2));
        m.insert("pomeranians", (Ordering::Equal, 3));
        m.insert("akitas", (Ordering::Equal, 0));
        m.insert("vizslas", (Ordering::Equal, 0));
        m.insert("goldfish", (Ordering::Equal, 5));
        m.insert("trees", (Ordering::Equal, 3));
        m.insert("cars", (Ordering::Equal, 2));
        m.insert("perfumes", (Ordering::Equal, 1));
        m
    };

    get_idx(hashmap, input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hashmap: HashMap<&'static str, (Ordering, usize)> = {
        let mut m = HashMap::new();
        m.insert("children", (Ordering::Equal, 3));
        m.insert("cats", (Ordering::Greater, 7));
        m.insert("samoyeds", (Ordering::Equal, 2));
        m.insert("pomeranians", (Ordering::Less, 3));
        m.insert("akitas", (Ordering::Equal, 0));
        m.insert("vizslas", (Ordering::Equal, 0));
        m.insert("goldfish", (Ordering::Less, 5));
        m.insert("trees", (Ordering::Greater, 3));
        m.insert("cars", (Ordering::Equal, 2));
        m.insert("perfumes", (Ordering::Equal, 1));
        m
    };

    get_idx(hashmap, input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
