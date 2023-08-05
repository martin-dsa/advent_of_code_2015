fn req_1(pw: &str) -> bool {
    pw.as_bytes()
        .windows(3)
        .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
}

fn req_2(pw: &str) -> bool {
    ['i', 'o', 'l'].iter().all(|c: &char| !pw.contains(*c))
}

fn req_3(pw: &str) -> bool {
    pw.as_bytes()
        .windows(2)
        .enumerate()
        .collect::<Vec<_>>()
        .iter()
        .filter(|(_, c)| c[0] == c[1])
        .collect::<Vec<_>>()[..]
        .windows(2)
        .any(|w| w[0].0 != w[1].0 - 1)
}

fn is_password_acceptable(pw: &str) -> bool {
    [req_1, req_2, req_3].iter().all(|f| f(pw))
}

struct Password(String);
impl Iterator for Password {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut add = 1;

        let r = unsafe { self.0.as_bytes_mut() };

        r.reverse();

        for c in &mut *r {
            if add == 1 && *c == b'z' {
                *c = b'a';
                add = 1;
            } else {
                *c += add;
                add = 0;
            }
        }

        r.reverse();

        Some(unsafe { String::from_utf8_unchecked(r.to_vec()) })
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut pw = input.trim().to_string();
    let mut pw_it = Password(pw.clone());

    while !is_password_acceptable(&pw) {
        pw = pw_it.next().unwrap();
    }

    Some(pw.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut pw = input.trim().to_string();
    let mut pw_it = Password(pw.clone());

    while !is_password_acceptable(&pw) {
        pw = pw_it.next().unwrap();
    }

    pw = pw_it.next().unwrap();

    while !is_password_acceptable(&pw) {
        pw = pw_it.next().unwrap();
    }

    Some(pw.to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some("abceffhh".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some("abceffjj".to_string()));
    }
}
