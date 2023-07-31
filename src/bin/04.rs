use md5::compute;

pub fn part_one(mut input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    input = input.trim();

    while {
        let md5 = compute(format!("{input}{result}"));
        let md5_string = format!("{:x}", md5);

        &md5_string[0..5] != "00000"
    } {
        result += 1
    }

    Some(result)
}

pub fn part_two(mut input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    input = input.trim();

    while {
        let md5 = compute(format!("{input}{result}"));
        let md5_string = format!("{:x}", md5);

        &md5_string[0..6] != "000000"
    } {
        result += 1
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(609043));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(6742839));
    }
}
