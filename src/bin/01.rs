pub fn part_one(input: &str) -> Option<u32> {
    let up = input.chars().filter(|c| *c == '(').count();
    let down = input.chars().filter(|c| *c == ')').count();
    Some((up - down) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cur_floor_pos = 0;

    for (idx, c) in input.chars().enumerate() {
        if cur_floor_pos == -1 {
            return Some(idx as u32);
        }

        if c == '(' {
            cur_floor_pos += 1;
        } else {
            cur_floor_pos -= 1;
        }
    }
    panic!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
