use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}
enum ParsingError {
    Command,
}

impl<'a> FromStr for Command<'a> {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s;
        if !x.starts_with("$ ") {
            return Err(ParsingError::Command);
        };
        x = &x[2..];
        match x[..2] {
            "ls" if x.len() == 2 => Command::Ls,
            "cd" => 
        }
    }
}


pub fn solve_part_one(input: &str) -> i32 {
    0
}

pub fn solve_part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example07.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 95437);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example07.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 19);
    }

    #[test]
    fn test_Command_paring() {
        let input = "$ cd /";
        let output = Command::Cd("/");
        assert_eq!(input.parse(), Ok(output));

        let input = "$ ls";
        let output = Command::Ls;
        assert_eq!(input.parse(), Ok(output));
    }
}
