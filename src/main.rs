use solvers::read_lines;
use std::{env, fmt::Display};

mod solvers;


enum Result {
    String(String),
    Int(i32),
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::String(x) => write!(f, "{}", x),
            Result::Int(x) => write!(f, "{}", x),
        }
    }
}

impl From<i32> for Result {
    fn from(x: i32) -> Self {
        Self::Int(x)
    }
}

impl From<String> for Result {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}

fn print_result(result: Result) {
    println!("-----------------------------");
    println!("Resunt is: {}", result);
    println!("-----------------------------")
}

macro_rules! solve {
    ($day: path, $input: expr) => {{
        use $day::*;
        print_result(solve_part_one($input).into());
        print_result(solve_part_two($input).into());
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();
    let input = read_lines(format!("input{:02}.in", day).as_str()).unwrap();

    match day {
        1 => solve!(solvers::day01, &input),
        2 => solve!(solvers::day02, &input),
        3 => solve!(solvers::day03, &input),
        4 => solve!(solvers::day04, &input),
        5 => solve!(solvers::day05, &input),
        6 => solve!(solvers::day06, &input),
        7 => solve!(solvers::day07, &input),
        _ => {
            unimplemented!("yet");
        }
    }
}
