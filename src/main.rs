extern crate core;

mod day15;
use day15::run;

const DAY: usize = 15;
const TESTING: bool = false;

fn fetch_input() -> String {
    let filename = format!("{}\\day{}.txt", (if TESTING {"tests"} else {"inputs"}), DAY);
    std::fs::read_to_string(&filename).expect(format!("Failed to read {}", &filename).as_str()).replace("\r", "")
}

fn main() {
    println!("Running day {}", DAY);
    let input = fetch_input();
    println!("Input: \n------\n{}\n------", input);
    let result = run(input);
    println!("Result: {:?}", result);
}
