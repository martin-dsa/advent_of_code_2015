use std::{convert::Infallible, str::FromStr, vec};

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]

struct Shop {
    weapons: [Item; 5],
    armors: [Item; 5],
    rings: [Item; 6],
}

#[derive(Clone, Debug)]
struct Unit {
    hp: i32,
    damage: u32,
    armor: u32,
    items: Vec<Item>,
}

impl FromStr for Unit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut params = s
            .lines()
            .map(|l| l.split_once(": ").unwrap().1.parse::<u32>().unwrap());
        Ok(Self {
            hp: params.next().unwrap() as i32,
            damage: params.next().unwrap(),
            armor: params.next().unwrap(),
            items: vec![],
        })
    }
}

impl Unit {
    fn get_damage(&self) -> u32 {
        self.damage + self.items.iter().map(|item| item.damage).sum::<u32>()
    }

    fn get_armor(&self) -> u32 {
        self.armor + self.items.iter().map(|item| item.armor).sum::<u32>()
    }

    fn fight_with(&mut self, other: &mut Self) -> bool {
        let damage_taken_enemy = self.get_damage() as i32 - other.get_armor() as i32;

        if damage_taken_enemy <= 0 {
            return false;
        }

        let damage_taken_player = (other.get_damage() as i32 - self.get_armor() as i32).max(0);

        loop {
            other.hp -= damage_taken_enemy;
            if !other.is_alive() {
                return true;
            }

            self.hp -= damage_taken_player;
            if !self.is_alive() {
                return false;
            }
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

const SHOP: Shop = Shop {
    weapons: [
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ],
    armors: [
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ],
    rings: [
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ],
};

const PLAYER: Unit = Unit {
    hp: 100,
    damage: 0,
    armor: 0,
    items: vec![],
};

fn get_item_combinations(shop: &Shop) -> Vec<Vec<Item>> {
    let weapons = shop.weapons;
    let mut armors = shop.armors.to_vec();
    armors.push(Item {
        cost: 0,
        damage: 0,
        armor: 0,
    });
    let mut rings = shop.rings.to_vec();
    rings.push(Item {
        cost: 0,
        damage: 0,
        armor: 0,
    });

    let mut item_combinations = vec![];

    for weapon in &weapons {
        for armor in &armors {
            for (idx, first_ring) in rings.iter().enumerate() {
                for second_ring in &rings[idx + 1..rings.len()] {
                    item_combinations.push(vec![*weapon, *armor, *first_ring, *second_ring]);
                }
            }
        }
    }
    item_combinations
}

pub fn part_one(input: &str) -> Option<u32> {
    let enemy = input.parse::<Unit>().unwrap();

    let item_combos = get_item_combinations(&SHOP);

    let min_cost_to_win = item_combos
        .iter()
        .filter_map(|items: &Vec<Item>| -> Option<u32> {
            let mut player = PLAYER.clone();
            player.items = items.clone();

            let fight_won = player.fight_with(&mut enemy.clone());

            match fight_won {
                false => None,
                true => Some(items.iter().map(|i| i.cost).sum::<u32>()),
            }
        })
        .min()
        .unwrap();

    Some(min_cost_to_win)
}

pub fn part_two(input: &str) -> Option<u32> {
    let enemy = input.parse::<Unit>().unwrap();

    let item_combos = get_item_combinations(&SHOP);

    let max_cost_to_lose = item_combos
        .iter()
        .filter_map(|items: &Vec<Item>| -> Option<u32> {
            let mut player = PLAYER.clone();
            player.items = items.clone();

            let fight_won = player.fight_with(&mut enemy.clone());

            match fight_won {
                false => Some(items.iter().map(|i| i.cost).sum::<u32>()),
                true => None,
            }
        })
        .max()
        .unwrap();

    Some(max_cost_to_lose)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
