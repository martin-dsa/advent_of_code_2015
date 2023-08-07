use lazy_static::lazy_static;
use regex::Regex;
use std::{convert::Infallible, str::FromStr};

lazy_static! {
    static ref RE: Regex =
        Regex::new(r".+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
}

const CAPACITY: usize = 0;
const DURABILITY: usize = 1;
const FLAVOR: usize = 2;
const TEXTURE: usize = 3;
const CALORIES: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Ingredient([i64; 5]);

impl FromStr for Ingredient {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RE.captures(s)
            .map(|c| {
                let (_, data) = c.extract();

                Ok(Self(data.map(|n| n.parse::<i64>().unwrap())))
            })
            .unwrap()
    }
}

fn ingr_it() -> impl Iterator<Item = [i64; 4]> {
    (0..100).flat_map(|ingr0| {
        (0..100 - ingr0).flat_map(move |ingr1| {
            (0..100 - ingr0 - ingr1).map(move |ingr2| {
                let ingr3 = 100 - ingr0 - ingr1 - ingr2;
                [ingr0, ingr1, ingr2, ingr3]
            })
        })
    })
}

fn get_sum_of_prop(ingredients: &Vec<Ingredient>, x: &[i64; 4], idx: usize) -> i64 {
    ingredients
        .iter()
        .map(|i| i.0[idx])
        .zip(x)
        .map(|(a, b)| a * b)
        .sum::<i64>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let ingredients = input
        .lines()
        .map(|l| l.parse::<Ingredient>().unwrap())
        .collect::<Vec<_>>();

    let max_val = ingr_it()
        .map(|ing_combo| {
            [CAPACITY, DURABILITY, FLAVOR, TEXTURE]
                .iter()
                .map(|idx| get_sum_of_prop(&ingredients, &ing_combo, *idx).max(0))
                .product::<i64>()
        })
        .max()
        .unwrap();

    Some(max_val as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ingredients = input
        .lines()
        .map(|l| l.parse::<Ingredient>().unwrap())
        .collect::<Vec<_>>();

    let max_val = ingr_it()
        .filter(|ing_combo| get_sum_of_prop(&ingredients, ing_combo, CALORIES) == 500)
        .map(|ing_combo| {
            [CAPACITY, DURABILITY, FLAVOR, TEXTURE]
                .iter()
                .map(|idx| get_sum_of_prop(&ingredients, &ing_combo, *idx).max(0))
                .product::<i64>()
        })
        .max()
        .unwrap();

    Some(max_val as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part_one() {
        todo!();
        // let input = advent_of_code::read_file("examples", 15);
        // assert_eq!(part_one(&input), Some(62842880));
    }

    #[test]
    fn test_part_two() {
        todo!();

        // let input = advent_of_code::read_file("examples", 15);
        // assert_eq!(part_two(&input), None);
    }
}
