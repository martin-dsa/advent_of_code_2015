type Row = [u8; 100];
type Grid = [Row; 100];

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            let array: Row = l.as_bytes().try_into().unwrap();
            array
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

const ON: u8 = b'#';
const OFF: u8 = b'.';

struct Point {
    x: i32,
    y: i32,
}

fn neighbor_indices(idx: Point) -> impl Iterator<Item = Point> {
    (idx.x - 1..=idx.x + 1)
        .flat_map(move |x| (idx.y - 1..=idx.y + 1).map(move |y| (x, y)))
        .filter_map(move |(x, y)| {
            if idx.x == x && idx.y == y {
                return None;
            }
            if x >= 0 && y >= 0 && x < 100 && y < 100 {
                Some(Point { x, y })
            } else {
                None
            }
        })
}

fn update_board(grid: &mut Grid) {
    let old_grid = *grid;
    for x in 0..100 {
        for y in 0..100 {
            let neighbor_lights_on = neighbor_indices(Point { x, y })
                .filter_map(|idx| {
                    let a = old_grid[idx.x as usize][idx.y as usize];
                    if a == ON {
                        Some(a)
                    } else {
                        None
                    }
                })
                .count();

            let light = &mut grid[x as usize][y as usize];

            *light = match (*light, neighbor_lights_on) {
                (ON, 2) | (ON, 3) | (OFF, 3) => ON,
                (ON, _) | (OFF, _) => OFF,
                _ => panic!(),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid = parse_grid(input);

    for _ in 0..100 {
        update_board(&mut grid);
    }
    let count_on = grid.iter().flatten().filter(|v| **v == ON).count() as u32;
    Some(count_on)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid = parse_grid(input);

    grid[0][0] = ON;
    grid[0][99] = ON;
    grid[99][0] = ON;
    grid[99][99] = ON;

    for _ in 0..100 {
        update_board(&mut grid);
        grid[0][0] = ON;
        grid[0][99] = ON;
        grid[99][0] = ON;
        grid[99][99] = ON;
    }
    let count_on = grid.iter().flatten().filter(|v| **v == ON).count() as u32;
    Some(count_on)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
