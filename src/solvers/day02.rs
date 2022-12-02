
#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lost,
}

impl From<&str> for Hand {
    fn from(str: &str) -> Self {
        match str {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("unknown hand shape!"),
        }
    }
}

impl From<&str> for Outcome {
    fn from(str: &str) -> Self {
        match str {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("unknown outcome!"),
        }
    }
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lost => 0,
        }
    }

    fn deduce_hand(&self, opponent: &Hand) -> Hand {
        match (self, opponent) {
            (Outcome::Win, Hand::Rock) => Hand::Paper,
            (Outcome::Win, Hand::Paper) => Hand::Scissors,
            (Outcome::Win, Hand::Scissors) => Hand::Rock,
            (Outcome::Draw, _) => *opponent,
            (Outcome::Lost, Hand::Rock) => Hand::Scissors,
            (Outcome::Lost, Hand::Paper) => Hand::Rock,
            (Outcome::Lost, Hand::Scissors) => Hand::Paper,
        }
    }
}

impl Hand {
    fn value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn round(&self, opponent: &Hand) -> Outcome {
        match (self, opponent) {
            (Hand::Rock, Hand::Paper) => Outcome::Lost,
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Scissors) => Outcome::Lost,
            (Hand::Scissors, Hand::Rock) => Outcome::Lost,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (_, _) => Outcome::Draw,
        }
    }

    fn round_points(&self, opponent: &Hand) -> i32 {
        self.value() + self.round(opponent).value()
    }

    fn round_points_with_outcome(opponent: &Hand, outcome: &Outcome) -> i32 {
        outcome.value() + outcome.deduce_hand(opponent).value()
    }
}

pub fn solve_part_one(input: &str) -> i32 {
    let lines = input.lines();
    lines
        .map(|l| l.split_once(' ').unwrap())
        .map(|(o, i)| Hand::from(i).round_points(&Hand::from(o)))
        .sum()
}

pub fn solve_part_two(input: &str) -> i32 {
    let lines = input.lines();
    lines
        .map(|l| l.split_once(' ').unwrap())
        .map(|(o, i)| Hand::round_points_with_outcome(&Hand::from(o), &Outcome::from(i)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example02.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 15);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example02.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 12);
    }
}
