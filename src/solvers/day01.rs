use std::{cmp::max, mem::swap};

pub fn solve_part_one(input: &str) -> i32 {
    let meals: Vec<Option<i32>> = input.lines().map(|i| i.parse().ok()).collect();
    let mut most_carried_calories = 0;
    let mut carried_calories = 0;
    for meal in meals {
        if let Some(calories) = meal {
            carried_calories += calories;
        } else {
            most_carried_calories = max(most_carried_calories, carried_calories);
            carried_calories = 0;
        }
    }
    most_carried_calories
}

pub fn solve_part_two(input: &str) -> i32 {
    let meals: Vec<Option<i32>> = input.lines().map(|i| i.parse().ok()).collect();
    let mut most_carried_calories = [0, 0, 0];
    let elves_calories: Vec<i32> = meals
        .split(Option::is_none)
        .map(|i| i.iter().map(|j| j.unwrap()).sum())
        .collect();
    for mut elf_calories in elves_calories {
        for most_carried_cal in most_carried_calories.iter_mut() {
            if elf_calories > *most_carried_cal {
                swap(&mut elf_calories,  most_carried_cal);
            }
        }
    }
    most_carried_calories.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example01.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 24000);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example01.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 45000);
    }
}
