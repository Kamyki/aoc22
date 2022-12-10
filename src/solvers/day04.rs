use std::{str::FromStr, num::ParseIntError};


struct Section(i32, i32);
#[derive(Debug)]
struct SectionParseError;

impl From<ParseIntError> for SectionParseError {
    fn from(_: ParseIntError) -> Self {
        SectionParseError
    }
}

impl FromStr for Section {
    type Err = SectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.split_once('-').ok_or(SectionParseError)?;
        let low_int = low.parse()?;
        let high_int = high.parse()?;
        Ok(Section(low_int, high_int))
    }
}

impl Section {

    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }
    
}

pub fn solve_part_one(input: &str) -> i32 {
    let lines = input.lines();
    let mut overlaps = 0;
    for line in lines {
        let (s1, s2) = line.split_once(',').unwrap();
        let section1: Section = s1.parse().unwrap();
        let section2: Section = s2.parse().unwrap();
        if section1.contains(&section2) || section2.contains(&section1) {
            overlaps += 1;
        }
    }
    overlaps
}

pub fn solve_part_two(input: &str) -> i32 {
    let lines = input.lines();
    let mut overlaps = 0;
    for line in lines {
        let (s1, s2) = line.split_once(',').unwrap();
        let section1: Section = s1.parse().unwrap();
        let section2: Section = s2.parse().unwrap();
        if section1.overlaps(&section2) {
            overlaps += 1;
        }
    }
    overlaps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example04.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example04.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 4);
    }
}
