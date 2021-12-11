mod day1;
mod day2;
mod utils;

fn main() {
    println!("Running day 1 problem 1");
    let result_day1_1 = day1::run1();
    println!("Result: {}", result_day1_1);

    println!("Running day 1 problem 2");
    let result_day1_2 = day1::run2();
    println!("Result: {}", result_day1_2);

    println!("Running day 2 problem 1");
    let result_day2_1 = day2::run1();
    println!("Result: {}", result_day2_1);

    println!("Running day 2 problem 2");
    let result_day2_2 = day2::run2();
    println!("Result: {}", result_day2_2);
}
