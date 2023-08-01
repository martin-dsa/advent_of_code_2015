use regex::Regex;

fn unescape(input: &str) -> usize {
    let mut res = input.len();
    res -= 2;

    let re = Regex::new(r#"\\\\\\""#).unwrap();
    res -= re.find_iter(input).count();

    let re = Regex::new(r#"\\\\""#).unwrap();
    res += re.find_iter(input).count();

    let re = Regex::new(r#"\\""#).unwrap();
    res -= re.find_iter(input).count();

    let re = Regex::new(r#"\\\\"#).unwrap();
    res -= re.find_iter(input).count();

    let re = Regex::new(r#"\\x[0-9a-f]{2}"#).unwrap();
    res -= re.find_iter(input).count() * 3;

    res
}

fn escape(input: &str) -> usize {
    let mut res = input.len();

    res += 4;

    let re = Regex::new(r#"\\\\\\""#).unwrap();
    res += re.find_iter(input).count() * 2;

    let re = Regex::new(r#"\\\\""#).unwrap();
    res -= re.find_iter(input).count() * 2;

    let re = Regex::new(r#"\\""#).unwrap();
    res += re.find_iter(input).count() * 2;

    let re = Regex::new(r#"\\\\"#).unwrap();
    res += re.find_iter(input).count() * 2;

    let re = Regex::new(r#"\\x[0-9a-f]{2}"#).unwrap();
    res += re.find_iter(input).count();

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(|l| l.len() - unescape(l)).sum::<usize>();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines().map(|l| escape(l) - l.len()).sum::<usize>();
    Some(result as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(19));
    }
}
