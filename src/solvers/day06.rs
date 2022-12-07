use std::{collections::VecDeque, iter::Enumerate};

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

struct InstructionStream<'a, const BUCKET_SIZE: usize> {
    iter: Enumerate<Graphemes<'a>>,
    bucket: VecDeque<&'a str>,
    found: bool,
}

impl<'a, const BUCKET_SIZE: usize> InstructionStream<'a, BUCKET_SIZE> {
    fn new(data: &'a str) -> Self {
        Self {
            iter: data.graphemes(true).enumerate(),
            bucket: VecDeque::new(),
            found: false,
        }
    }
}

impl<'a, const BUCKET_SIZE: usize> Iterator for InstructionStream<'a, BUCKET_SIZE> {
    type Item = Option<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found {
            return None;
        }
        if let Some((i, x)) = self.iter.next() {
            //dbg!(&self.bucket, i);
            if self.bucket.contains(&x) {
                while let Some(r) = self.bucket.pop_front() {
                    if r == x {
                        break;
                    }
                }
                self.bucket.push_back(x);
                return Some(None);
            } else {
                self.bucket.push_back(x);
            }

            if self.bucket.len() < BUCKET_SIZE {
                return Some(None);
            } else {
                self.found = true;
                return Some(Some(i as i32 + 1));
            }
        } else {
            None
        }
    }
}

pub fn solve_part_one(input: &str) -> i32 {
    let instructions: InstructionStream<4> = InstructionStream::new(input.lines().next().unwrap());
    for x in instructions {
        match x {
            Some(c) => return c,
            None => continue,
        }
    }
    0
}

pub fn solve_part_two(input: &str) -> i32 {
    let instructions: InstructionStream<14> = InstructionStream::new(input.lines().next().unwrap());
    for x in instructions {
        match x {
            Some(c) => return c,
            None => continue,
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_lines;

    #[test]
    fn test_part_one() {
        let input = read_lines("example06-1.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 7);
        let input = read_lines("example06-2.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 5);
        let input = read_lines("example06-3.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 6);
        let input = read_lines("example06-4.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 10);
        let input = read_lines("example06-5.in").unwrap();
        let res = solve_part_one(&input);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("example06-1.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 19);
        let input = read_lines("example06-2.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 23);
        let input = read_lines("example06-3.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 23);
        let input = read_lines("example06-4.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 29);
        let input = read_lines("example06-5.in").unwrap();
        let res = solve_part_two(&input);
        assert_eq!(res, 26);
    }
}
