pub fn run1() -> u16 {
    let lines = super::utils::read_file("src/day1/input.txt", false);

    let mut last_number: u16 = 0;
    let mut increments: u16 = 0;

    for line in lines {
        let number = line.parse::<u16>().unwrap();
        if number > last_number && last_number > 0 {
            increments += 1;
        }
        last_number = number;
    }
    return increments;
}

pub fn run2() -> usize {
    let lines = super::utils::read_file("src/day1/input.txt", false);

    let last_index = lines.len() - 3;

    let mut last_number: usize = 0;

    let mut total_increases: usize = 0;

    for i in 0..last_index {
        let number = lines[i].parse::<usize>().unwrap();
        let next_number = lines[i + 1].parse::<usize>().unwrap();
        let next_next_number = lines[i + 2].parse::<usize>().unwrap();
        let sum = number + next_number + next_next_number;
        if sum > last_number {
            total_increases += 1;
        }
        last_number = sum;
    }

    return total_increases;
}
