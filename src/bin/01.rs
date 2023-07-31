pub fn part_one(input: &str) -> Option<i32> {
    let up = input.chars().filter(|c| *c == '(').count() as i32;
    let down = input.chars().filter(|c| *c == ')').count() as i32;
    Some(up - down)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cur_floor_pos = 0;

    for (idx, c) in input.chars().enumerate() {
        if c == '(' {
            cur_floor_pos += 1;
        } else {
            cur_floor_pos -= 1;
        }

        if cur_floor_pos == -1 {
            return Some((idx + 1) as i32);
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
        assert_eq!(part_one(&input), Some(-1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
