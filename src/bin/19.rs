use std::collections::HashSet;

use regex::{Match, Regex};

fn parse(s: &str) -> (impl Iterator<Item = (&str, &str)>, &str) {
    let (replacements, molecule) = s
        .split_once("\n\n")
        .map(|(replacements, molecule)| {
            (
                replacements.lines().map(|l| l.split_once(" => ").unwrap()),
                molecule.trim(),
            )
        })
        .unwrap();
    (replacements, molecule)
}

fn new_molecule(prev_molecule: &str, m: Match, replacement: &str) -> String {
    format!(
        "{}{}{}",
        &prev_molecule[0..m.start()],
        replacement,
        &prev_molecule[m.end()..prev_molecule.len()]
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (replacements, molecule) = parse(input);

    let new_molecules = replacements
        .flat_map(|r| {
            Regex::new(r.0)
                .unwrap()
                .find_iter(molecule)
                .map(|m| new_molecule(molecule, m, r.1))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    Some(new_molecules.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (replacements, molecule) = parse(input);

    let mut molecule = molecule.to_string();
    let replacements = replacements.collect::<Vec<_>>();
    let mut count = 0;

    while molecule != *"e" {
        for r in replacements.iter() {
            if let Some(m) = Regex::new(r.1).unwrap().find(molecule.clone().as_str()) {
                count += 1;

                molecule = new_molecule(&molecule, m, r.0)
            }
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
