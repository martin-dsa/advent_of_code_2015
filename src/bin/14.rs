use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, convert::Infallible, str::FromStr};

lazy_static! {
    static ref RE: Regex = Regex::new(
        r".* can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds."
    )
    .unwrap();
}

const TIME_TO_TRAVEL: u32 = 2503;

#[derive(Eq, PartialEq, Hash)]
struct Deer {
    speed_km_per_s: u32,
    fly_time: u32,
    rest_time: u32,
}

impl FromStr for Deer {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RE.captures(s)
            .map(|x| {
                let (_, data) = x.extract();
                let [speed_km_per_s, fly_time, rest_time] = data.map(|x| x.parse::<u32>().unwrap());
                Ok(Self {
                    speed_km_per_s,
                    fly_time,
                    rest_time,
                })
            })
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let deers = input
        .lines()
        .map(|l| l.parse::<Deer>().unwrap())
        .collect::<Vec<_>>();

    let mut scores = deers
        .iter()
        .map(|d| (d, 0))
        .collect::<HashMap<&Deer, u32>>();

    for s in 0..TIME_TO_TRAVEL {
        for deer in &deers {
            if s % (deer.fly_time + deer.rest_time) < deer.fly_time {
                *scores.get_mut(&deer).unwrap() += deer.speed_km_per_s;
            }
        }
    }

    scores.values().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let deers = input
        .lines()
        .map(|l| l.parse::<Deer>().unwrap())
        .collect::<Vec<_>>();

    let mut scores = deers
        .iter()
        .map(|d| (d, 0))
        .collect::<HashMap<&Deer, u32>>();

    let mut cur_dist = scores.clone();

    for sec in 0..TIME_TO_TRAVEL {
        for deer in &deers {
            if sec % (deer.fly_time + deer.rest_time) < deer.fly_time {
                *cur_dist.get_mut(&deer).unwrap() += deer.speed_km_per_s;
            }
        }

        let cur_leader = cur_dist.values().max().copied().unwrap();

        for (deer, dist) in &cur_dist {
            if *dist == cur_leader {
                *scores.get_mut(deer).unwrap() += 1;
            }
        }
    }

    scores.values().max().copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(2660));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(1564));
    }
}
