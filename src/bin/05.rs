use std::str::from_utf8;

fn rule1_1(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.chars().filter(|c| vowels.contains(c)).count() >= 3
}

fn rule2_1(s: &str) -> bool {
    s.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn rule3_1(s: &str) -> bool {
    ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|disallowed: &&str| s.contains(disallowed))
}

fn is_nice(s: &str) -> Option<&str> {
    (rule1_1(s) && rule2_1(s) && !rule3_1(s)).then_some(s)
}

fn rule1_2(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .enumerate()
        .any(|(idx, w)| s[idx + 2..].contains(from_utf8(w).unwrap()))
}

fn rule2_2(s: &str) -> bool {
    s.as_bytes().windows(3).any(|w| w[0] == w[2])
}
fn is_nice_2(s: &str) -> Option<&str> {
    (rule1_2(s) && rule2_2(s)).then_some(s)
}

pub fn part_one(input: &str) -> Option<u32> {
    let nice_count = input.lines().filter_map(is_nice).count() as u32;

    Some(nice_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nice_count = input.lines().filter_map(is_nice_2).count() as u32;

    Some(nice_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(0));
    }
}
