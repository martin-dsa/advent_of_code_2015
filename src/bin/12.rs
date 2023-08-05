use serde_json::{Map, Value};

fn get_sum_obj(obj: &Map<String, Value>) -> i64 {
    obj.iter().map(|(_k, v)| get_sum(v, get_sum_obj)).sum()
}

fn get_sum_obj_2(obj: &Map<String, Value>) -> i64 {
    let contains_red = obj.values().any(|v| *v == Value::String("red".to_string()));
    if contains_red {
        0
    } else {
        obj.iter().map(|(_k, v)| get_sum(v, get_sum_obj_2)).sum()
    }
}

fn get_sum(value: &Value, get_obj_sum: fn(obj: &Map<String, Value>) -> i64) -> i64 {
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|v| get_sum(v, get_obj_sum)).sum(),
        Value::Object(obj) => get_obj_sum(obj),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let value: Value = serde_json::from_str(input).unwrap();
    let sum = get_sum(&value, get_sum_obj);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let value: Value = serde_json::from_str(input).unwrap();
    let sum = get_sum(&value, get_sum_obj_2);
    Some(sum as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(10));
    }
}
