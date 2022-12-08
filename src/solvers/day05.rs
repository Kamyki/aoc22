use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Crate(char);

#[derive(Debug, PartialEq, Eq)]
enum Slot {
    Empty,
    Crate(Crate),
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    ParseCrateError,
    ParseInstructionError,
    ParseLineError,
}

impl From<Crate> for Slot {
    fn from(x: Crate) -> Self {
        Self::Crate(x)
    }
}

impl FromStr for Slot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "   " => Ok(Self::Empty),
            _ => s.parse::<Crate>().map(Self::from),
        }
    }
}

type Stack = VecDeque<Crate>;

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct SupplyStacks {
    stacks: HashMap<usize, Stack>,
    instructions: Vec<Instruction>,
}

impl FromStr for SupplyStacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks: HashMap<usize, VecDeque<Crate>> = HashMap::new();
        let mut lines = s.lines();

        while let Some(line) = lines.next() {
            if !line.starts_with(" 1") {
                let mut i = 0;
                while i * 4 + 2 < line.len() {
                    let slot = line[i * 4..i * 4 + 3].parse::<Slot>()?;
                    if let Slot::Crate(c) = slot {
                        stacks.entry(i + 1).or_default().push_front(c);
                    }
                    i += 1;
                }
            } else {
                break;
            }
        }
        match lines.next() {
            Some("") => Ok(()),
            _ => Err(ParseError::ParseLineError),
        }?;

        let instructions = lines
            .map(Instruction::from_str)
            .collect::<Result<Vec<Instruction>, ParseError>>()?;

        Ok(Self {
            stacks,
            instructions,
        })
    }
}

impl FromStr for Crate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if let Some('[') = chars.next() {
            Ok(())
        } else {
            Err(Self::Err::ParseCrateError)
        }?;
        let n = chars.next();
        if let Some(']') = chars.next() {
            Ok(())
        } else {
            Err(Self::Err::ParseCrateError)
        }?;
        if let None = chars.next() {
            Ok(())
        } else {
            Err(Self::Err::ParseCrateError)
        }?;
        Ok(Self(n.unwrap()))
    }
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseInstructionError
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        let mut s = x;

        if s.starts_with("move ") {
            Ok(())
        } else {
            Err(Self::Err::ParseInstructionError)
        }?;
        s = &s[5..];

        let space = s.find(" ").ok_or(Self::Err::ParseInstructionError)?;
        let amount: usize = s[..space].parse()?;
        s = &s[space + 1..];

        if let Some(0) = s.find("from ") {
            Ok(())
        } else {
            Err(Self::Err::ParseInstructionError)
        }?;
        s = &s[5..];

        let space = s.find(" ").ok_or(Self::Err::ParseInstructionError)?;
        let from: usize = s[..space].parse()?;
        s = &s[space + 1..];

        if let Some(0) = s.find("to ") {
            Ok(())
        } else {
            Err(Self::Err::ParseInstructionError)
        }?;
        s = &s[3..];

        let to: usize = s.parse()?;
        Ok(Self { amount, from, to })
    }
}

impl SupplyStacks {
    fn simulate(&mut self) -> String {
        for instr in self.instructions.drain(..) {
            let from = self.stacks.get_mut(&instr.from).unwrap();
            let s = from.split_off(from.len() - instr.amount);
            let stack = self.stacks.get_mut(&instr.to).unwrap();
            stack.extend(s.into_iter().rev());
        }
        let x = (1..=self.stacks.len())
            .map(|i| self.stacks.get(&i))
            .map(|m| m.and_then(|v| v.back()))
            .map(|c| c.unwrap().0)
            .join("");
        x
    }

    fn simulate2(&mut self) -> String {
        for instr in self.instructions.drain(..) {
            let from = self.stacks.get_mut(&instr.from).unwrap();
            let s = from.split_off(from.len() - instr.amount);
            let stack = self.stacks.get_mut(&instr.to).unwrap();
            stack.extend(s);
        }
        let x = (1..=self.stacks.len())
            .map(|i| self.stacks.get(&i))
            .map(|m| m.and_then(|v| v.back()))
            .map(|c| c.unwrap().0)
            .join("");
        x
    }
}

pub fn solve_part_one(input: &str) -> String {
    let mut supply_stacks = SupplyStacks::from_str(input).unwrap();
    supply_stacks.simulate()
}

pub fn solve_part_two(input: &str) -> String {
    let mut supply_stacks = SupplyStacks::from_str(input).unwrap();
    supply_stacks.simulate2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example05.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, "CMZ");
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example05.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, "MCD");
    }

    #[test]
    fn test_instruction_from_str_1() {
        let input = "move 1 from 1 to 2";
        let res: Instruction = input.parse().unwrap();
        assert_eq!(
            res,
            Instruction {
                amount: 1,
                from: 1,
                to: 2
            }
        );
    }

    #[test]
    fn test_instruction_from_str_2() {
        let input = "move 12 from 1123 to 223";
        let res: Instruction = input.parse().unwrap();
        assert_eq!(
            res,
            Instruction {
                amount: 12,
                from: 1123,
                to: 223
            }
        );
    }

    #[test]
    fn test_instruction_from_str_3() {
        let input = "move 1 from 1 to 2 ";
        let res = input.parse::<Instruction>();
        assert_eq!(res, Err(ParseError::ParseInstructionError),);
    }

    #[test]
    fn test_crate_from_str_1() {
        let input = "[1]";
        let res = input.parse::<Crate>();
        assert_eq!(res, Ok(Crate('1')));
    }

    #[test]
    fn test_crate_from_str_2() {
        let input = "[12]";
        let res = input.parse::<Crate>();
        assert_eq!(res, Err(ParseError::ParseCrateError));
    }

    #[test]
    fn test_crate_from_str_3() {
        let input = "[1] ";
        let res = input.parse::<Crate>();
        assert_eq!(res, Err(ParseError::ParseCrateError));
    }

    #[test]
    fn test_seacks_from_str_1() {
        let input = read_lines("example05.in").unwrap();
        let res = input.parse::<SupplyStacks>();

        let stacks: Vec<(usize, VecDeque<_>)> = vec![
            (1, vec![Crate('N'), Crate('Z')].into()),
            (2, vec![Crate('D'), Crate('C'), Crate('M')].into()),
            (3, vec![Crate('P')].into()),
        ];
        let supply_stacks = SupplyStacks {
            stacks: stacks.into_iter().collect::<HashMap<usize, _>>(),
            instructions: vec![
                Instruction {
                    amount: 1,
                    from: 2,
                    to: 1,
                },
                Instruction {
                    amount: 3,
                    from: 1,
                    to: 3,
                },
                Instruction {
                    amount: 2,
                    from: 2,
                    to: 1,
                },
                Instruction {
                    amount: 1,
                    from: 1,
                    to: 2,
                },
            ],
        };
        assert_eq!(res, Ok(supply_stacks));
    }
}
