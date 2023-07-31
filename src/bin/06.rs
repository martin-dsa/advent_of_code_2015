use std::{convert::Infallible, str::FromStr};

enum ActionType {
    ON,
    OFF,
    TOGGLE,
}
impl FromStr for ActionType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "on" => ActionType::ON,
            "off" => ActionType::OFF,
            _ => panic!(),
        })
    }
}
struct Pos {
    x: usize,
    y: usize,
}
impl FromStr for Pos {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(',')
            .map(|(x, y)| {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                Ok(Self { x, y })
            })
            .unwrap()
    }
}
struct Action {
    t: ActionType,
    from: Pos,
    to: Pos,
}

impl FromStr for Action {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        match words.len() {
            4 => Ok(Self {
                t: ActionType::TOGGLE,
                from: words[1].parse().unwrap(),
                to: words[3].parse().unwrap(),
            }),
            5 => Ok(Self {
                t: words[1].parse().unwrap(),
                from: words[2].parse().unwrap(),
                to: words[4].parse().unwrap(),
            }),

            _ => panic!(),
        }
    }
}

struct Grid<T>([[T; 1000]; 1000]);

trait Actionable {
    type T;

    fn get_item(&mut self, x: usize, y: usize) -> &mut Self::T;

    fn on_on(v: &mut Self::T) -> ();
    fn on_off(v: &mut Self::T) -> ();
    fn on_toggle(v: &mut Self::T) -> ();

    fn do_action(&mut self, action: &Action) {
        let func = match action.t {
            ActionType::ON => Self::on_on,
            ActionType::OFF => Self::on_off,
            ActionType::TOGGLE => Self::on_toggle,
        };

        for x in action.from.x..=action.to.x {
            for y in action.from.y..=action.to.y {
                func(self.get_item(x, y))
            }
        }
    }
}
impl Actionable for Grid<bool> {
    type T = bool;

    fn get_item(&mut self, x: usize, y: usize) -> &mut Self::T {
        &mut self.0[x][y]
    }

    fn on_on(v: &mut Self::T) -> () {
        *v = true
    }

    fn on_off(v: &mut Self::T) -> () {
        *v = false
    }

    fn on_toggle(v: &mut Self::T) -> () {
        *v = !*v
    }
}
impl Grid<bool> {
    fn new() -> Self {
        Grid([[false; 1000]; 1000])
    }

    fn count_on(&self) -> usize {
        let mut res = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if self.0[x][y] == true {
                    res += 1;
                }
            }
        }
        res
    }
}

impl Actionable for Grid<i32> {
    type T = i32;

    fn get_item(&mut self, x: usize, y: usize) -> &mut Self::T {
        &mut self.0[x][y]
    }

    fn on_on(v: &mut Self::T) -> () {
        *v += 1
    }

    fn on_off(v: &mut Self::T) -> () {
        *v = (*v - 1).max(0)
    }

    fn on_toggle(v: &mut Self::T) -> () {
        *v += 2
    }
}

impl Grid<i32> {
    fn new() -> Self {
        Grid([[0; 1000]; 1000])
    }

    fn count_on(&self) -> usize {
        let mut res = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                res += self.0[x][y] as usize;
            }
        }
        res
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::<bool>::new();

    let actions = input.lines().map(|line| line.parse::<Action>().unwrap());
    for action in actions {
        grid.do_action(&action);
    }
    Some(grid.count_on() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::<i32>::new();

    let actions = input.lines().map(|line| line.parse::<Action>().unwrap());
    for action in actions {
        grid.do_action(&action);
    }
    Some(grid.count_on() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(998996));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(1));
    }
}
