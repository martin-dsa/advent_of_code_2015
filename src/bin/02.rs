use std::{convert::Infallible, str::FromStr};

struct PresentBox {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for PresentBox {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.split('x').map(|s| s.parse::<u32>().unwrap());

        Ok(Self {
            l: dimensions.next().unwrap(),
            w: dimensions.next().unwrap(),
            h: dimensions.next().unwrap(),
        })
    }
}

impl PresentBox {
    fn paper(&self) -> u32 {
        let Self { l, w, h } = self;

        let areas = [l * w, w * h, h * l];
        let smallest_area = areas.iter().min().unwrap();

        areas.iter().map(|a| *a * 2).sum::<u32>() + smallest_area
    }

    fn paper_2(&self) -> u32 {
        let Self { l, w, h } = self;

        let perimeters = [l + l + w + w, w + w + h + h, h + h + l + l];
        let smallest_perimeter = perimeters.iter().min().unwrap();

        l * w * h + smallest_perimeter
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_paper = input
        .lines()
        .map(|line| {
            let present_box = line.parse::<PresentBox>().unwrap();
            present_box.paper()
        })
        .sum();

    Some(total_paper)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_paper = input
        .lines()
        .map(|line| {
            let present_box = line.parse::<PresentBox>().unwrap();
            present_box.paper_2()
        })
        .sum();

    Some(total_paper)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(52));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(10));
    }
}
