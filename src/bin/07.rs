use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug)]
enum Operation {
    RightShift(String, u16),
    LeftShift(String, u16),
    Or(String, String),
    And(String, String),
    Not(String),
    NoopW(String),
    NoopN(u16),
}

impl FromStr for Operation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl = s.split(' ').collect::<Vec<_>>();
        Ok(match spl[..] {
            [a, "RSHIFT", b] => Self::RightShift(a.to_string(), b.parse().unwrap()),
            [a, "LSHIFT", b] => Self::LeftShift(a.to_string(), b.parse().unwrap()),
            [a, "OR", b] => Self::Or(a.to_string(), b.to_string()),
            [a, "AND", b] => Self::And(a.to_string(), b.to_string()),
            ["NOT", a] => Self::Not(a.to_string()),
            [a] => match a.parse::<u16>() {
                Ok(n) => Self::NoopN(n),
                Err(_) => Self::NoopW(a.to_string()),
            },

            _ => panic!(),
        })
    }
}

fn solve(
    wire: &String,
    steps: &HashMap<String, Operation>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    let res = cache.get(wire);
    if let Some(r) = res {
        return *r;
    }
    let step = steps.get(wire).unwrap();

    let a = match step {
        Operation::RightShift(a, b) => solve(a, steps, cache) >> b,
        Operation::LeftShift(a, b) => solve(a, steps, cache) << b,
        Operation::Or(a, b) => solve(a, steps, cache) | solve(b, steps, cache),
        Operation::And(a, b) => match (a.as_str(), b) {
            ("1", b) => 1 & solve(b, steps, cache),
            (a, b) => solve(&a.to_string(), steps, cache) & solve(b, steps, cache),
        },
        Operation::Not(a) => !solve(a, steps, cache),
        Operation::NoopW(a) => solve(a, steps, cache),
        Operation::NoopN(a) => *a,
    };

    cache.insert(wire.clone(), a);
    a
}

fn part_one(input: &str) -> Option<u32> {
    let operations = input
        .lines()
        .map(|line| {
            let (op, to) = line.split_once(" -> ").unwrap();
            let operation = op.parse::<Operation>().unwrap();
            (to.to_string(), operation)
        })
        .collect::<HashMap<_, _>>();

    let mut cache: HashMap<String, u16> = HashMap::new();

    let a_wire = solve(&"a".to_string(), &operations, &mut cache);

    Some(a_wire as u32)
}

fn part_two(input: &str) -> Option<u32> {
    let mut operations = input
        .lines()
        .map(|line| {
            let (op, to) = line.split_once(" -> ").unwrap();
            let operation = op.parse::<Operation>().unwrap();
            (to.to_string(), operation)
        })
        .collect::<HashMap<_, _>>();
    let mut cache: HashMap<String, u16> = HashMap::new();
    let a_wire: u16 = solve(&"a".to_string(), &operations, &mut cache);

    *operations.get_mut("b").unwrap() = Operation::NoopN(a_wire);

    let mut cache: HashMap<String, u16> = HashMap::new();
    let a_wire: u16 = solve(&"a".to_string(), &operations, &mut cache);

    Some(a_wire as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(123));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(123));
    }
}
