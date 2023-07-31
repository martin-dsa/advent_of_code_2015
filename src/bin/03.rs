use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_(&mut self, dir: char) {
        match dir {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => (),
        };
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut current_pos = Position { x: 0, y: 0 };
    visited.insert(current_pos);

    for dir in input.chars() {
        current_pos.move_(dir);
        visited.insert(current_pos);
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut current_pos_santa = Position { x: 0, y: 0 };
    visited.insert(current_pos_santa);
    let mut current_pos_robot = Position { x: 0, y: 0 };
    visited.insert(current_pos_robot);

    for (idx, dir) in input.chars().enumerate() {
        let position = if idx % 2 == 0 {
            &mut current_pos_santa
        } else {
            &mut current_pos_robot
        };

        position.move_(dir);
        visited.insert(*position);
    }
    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(11));
    }
}
