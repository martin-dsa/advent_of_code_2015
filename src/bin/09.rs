use std::collections::{HashMap, HashSet};

type Path = Vec<String>;

fn get_distances(input: &str) -> (HashMap<(&str, &str), usize>, HashSet<&str>) {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for line in input.lines() {
        let mut a = line.split_whitespace();
        let loc1 = a.next().unwrap();
        a.next();
        let loc2 = a.next().unwrap();
        a.next();
        let dist = a.next().unwrap().parse::<usize>().unwrap();

        map.insert((loc1, loc2), dist);
        map.insert((loc2, loc1), dist);

        set.insert(loc1);
        set.insert(loc2);
    }

    (map, set)
}

fn get_all_possible_paths(
    all_paths: &mut Vec<Path>,
    cur_path: Path,
    locations_to_visit: &HashSet<&str>,
) {
    if locations_to_visit.is_empty() {
        all_paths.push(cur_path);
        return;
    }

    for loc in locations_to_visit {
        let mut next_path = cur_path.clone();
        next_path.push(loc.to_string());

        let mut other = locations_to_visit.clone();
        other.remove(loc);

        get_all_possible_paths(all_paths, next_path, &other);
    }
}

fn get_all_distances(input: &str) -> Vec<usize> {
    let (distances, locations) = get_distances(input);

    let mut paths = vec![];
    get_all_possible_paths(&mut paths, vec![], &locations);

    let result = paths
        .iter()
        .map(|path| {
            path[..]
                .windows(2)
                .map(|pair| {
                    distances
                        .get(&(pair[0].as_str(), pair[1].as_str()))
                        .unwrap()
                })
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    get_all_distances(input).iter().min().map(|res| *res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_all_distances(input).iter().max().map(|res| *res as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(982));
    }
}
