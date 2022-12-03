use solvers::read_lines;
use std::env;

mod solvers;
fn print_result(result: i32) {
    println!("-----------------------------");
    println!("Resunt is: {}", result);
    println!("-----------------------------")
}

macro_rules! solve {
    ($day: path, $input: expr) => {{
        use $day::*;
        print_result(solve_part_one($input));
        print_result(solve_part_two($input));
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
        _ => {
            unimplemented!("yet");
        }
    }
}
