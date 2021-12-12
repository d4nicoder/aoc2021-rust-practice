mod day1;
mod day2;
mod utils;

fn main() {
    print!("Running day 1 problem 1: ");
    let result_day1_1 = day1::run1();
    println!("{}", result_day1_1);

    print!("Running day 1 problem 2: ");
    let result_day1_2 = day1::run2();
    println!("{}", result_day1_2);

    print!("Running day 2 problem 1: ");
    let result_day2_1 = day2::run1();
    println!("{}", result_day2_1);

    print!("Running day 2 problem 2: ");
    let result_day2_2 = day2::run2();
    println!("{}", result_day2_2);
}
