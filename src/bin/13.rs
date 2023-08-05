use itertools::Itertools;

use std::collections::{HashMap, HashSet};

type Names<'a> = HashSet<&'a str>;
type Connections<'a> = HashMap<(&'a str, &'a str), i32>;

fn get_data(input: &str) -> (Names, Connections) {
    let mut names: Names = HashSet::new();

    let connections = input
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();

            let from = &words.last().unwrap()[0..words.last().unwrap().len() - 1];
            let to = *words.first().unwrap();
            names.insert(to.clone());

            let sign = if words[2] == "gain" { 1 } else { -1 };
            let value = words[3].parse::<i32>().unwrap() * sign;

            ((from, to), value)
        })
        .collect::<Connections>();
    (names, connections)
}

fn get_score(permutation: Vec<&&str>, connections: &Connections) -> i32 {
    let first = permutation.first().unwrap();
    let last = permutation.last().unwrap();

    let mut temp_people: Vec<&&str> = vec![];

    temp_people.push(last);
    temp_people.append(&mut permutation.clone());
    temp_people.push(first);

    temp_people
        .windows(3)
        .map(|w| {
            let person = w[1];

            let neighbor_left = w[0];
            let neighbor_right = w[2];

            connections.get(&(neighbor_left, person)).unwrap()
                + connections.get(&(neighbor_right, person)).unwrap()
        })
        .sum()
}

fn add_myself<'a>(
    names: &'a mut Names,
    connections: &'a mut Connections,
) -> (Names<'a>, Connections<'a>) {
    let my_name = "Martin";

    let mut people = names.clone();
    let mut connections = connections.clone();

    for person in people.iter() {
        connections.insert((my_name, person), 0);
        connections.insert((person, my_name), 0);
    }

    people.insert(my_name);

    (people, connections)
}

fn get_max_score(names: &Names, connections: &Connections) -> i32 {
    names
        .iter()
        .permutations(names.len())
        .unique()
        .map(|p| get_score(p, &connections))
        .max()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (names, connections) = get_data(input);

    let max_score = get_max_score(&names, &connections);

    Some(max_score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut names, mut connections) = get_data(input);

    let (names, connections) = add_myself(&mut names, &mut connections);

    let max_score = get_max_score(&names, &connections);

    Some(max_score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(330));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(286));
    }
}
