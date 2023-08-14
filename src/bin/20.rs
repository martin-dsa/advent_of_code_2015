pub fn part_one(input: &str) -> Option<u32> {
    let presents: usize = input.trim().parse::<usize>().unwrap();

    let mut houses: Vec<u32> = vec![0; presents / 10];

    for elf in 1..(houses.len()) {
        for house_idx in (elf..houses.len()).step_by(elf) {
            houses[house_idx] += (elf * 10) as u32;
        }

        if presents <= houses[elf] as usize {
            return Some(elf as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let presents: usize = input.trim().parse::<usize>().unwrap();

    let mut houses: Vec<u32> = vec![0; presents / 11];

    for elf in 1..(houses.len()) {
        for house_idx in (elf..houses.len()).step_by(elf).take(50) {
            houses[house_idx] += (elf * 11) as u32;
        }

        if presents <= houses[elf] as usize {
            return Some(elf as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
