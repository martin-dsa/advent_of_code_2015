use lazy_static::lazy_static;

use fancy_regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d)\1*").unwrap();
}

fn new_string(input: String) -> String {
    RE.find_iter(&input)
        .flat_map(|m| {
            let match_ = m.unwrap().as_str();

            let count = match_.len().to_string();
            let number = match_.chars().next().unwrap().to_string();
            [count, number]
        })
        .collect::<String>()
}

fn get_result(input: &str, number_of_iterations: usize) -> Option<u32> {
    let mut s = input.to_string();
    for _ in 0..number_of_iterations {
        s = new_string(s);
    }
    Some(s.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    get_result(input, 40)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_result(input, 50)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(124496));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(1766402));
    }
}
