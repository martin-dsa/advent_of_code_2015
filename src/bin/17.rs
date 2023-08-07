use itertools::Itertools;

fn combinations(containers: &Vec<u32>) -> impl Iterator<Item = Vec<&u32>> {
    (1..containers.len()).flat_map(|len| containers.iter().combinations(len))
}

pub fn part_one(input: &str) -> Option<u32> {
    let containers = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let count = combinations(&containers)
        .filter(|c| c.iter().copied().sum::<u32>() == 150)
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let containers = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let min = combinations(&containers)
        .filter(|c| c.iter().copied().sum::<u32>() == 150)
        .group_by(|c| c.len())
        .into_iter()
        .map(|(_, g)| g.count())
        .min()
        .unwrap();

    Some(min as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
