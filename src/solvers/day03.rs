use std::collections::{hash_map::RandomState, HashSet};

use itertools::Itertools;

fn priority(ch: &char) -> i32 {
    if ch.is_alphabetic() {
        if ch.is_ascii_lowercase() {
            *ch as i32 - 96
        } else {
            *ch as i32 - 38
        }
    } else {
        panic!("Unknown item type")
    }
}

pub fn solve_part_one(input: &str) -> i32 {
    let lines = input.lines();
    let mut priority_sum = 0;
    for line in lines {
        let (first_part, second_part) = line.split_at(line.len() / 2);
        let first: HashSet<char, RandomState> = HashSet::from_iter(first_part.chars());
        let second: HashSet<char, RandomState> = HashSet::from_iter(second_part.chars());
        let interection = first.intersection(&second);
        priority_sum += interection.map(priority).sum::<i32>()
    }
    priority_sum
}

pub fn solve_part_two(input: &str) -> i32 {
    let mut priority_sum: i32 = 0;
    for mut group in &input
        .lines()
        .map(str::chars)
        .map(HashSet::from_iter)
        .chunks(3)
    {
        let (g1, g2, g3) = group.next_tuple().unwrap();
        priority_sum += g1
            .intersection(&g2)
            .cloned()
            .collect::<HashSet<char, RandomState>>()
            .intersection(&g3)
            .map(priority)
            .sum::<i32>();
    }
    priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example03.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 157);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example03.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 70);
    }
}
