pub fn run1() -> usize {
    let lines = super::utils::read_file("src/day2/input.txt", true);
    let mut x: usize = 0;
    let mut depth: usize = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let content = line.split(" ").collect::<Vec<&str>>();
        let quantity: usize = content[1].parse::<usize>().unwrap();
        let instruction = content[0].parse::<String>().unwrap();

        if instruction.eq("forward") {
            x += quantity;
        } else if instruction.eq("up") {
            depth -= quantity;
        } else if instruction.eq("down") {
            depth += quantity;
        }
    }
    return x * depth;
}

pub fn run2() -> usize {
    let lines = super::utils::read_file("src/day2/input.txt", true);
    let mut x: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let content = line.split(" ").collect::<Vec<&str>>();
        let quantity: usize = content[1].parse::<usize>().unwrap();
        let instruction = content[0].parse::<String>().unwrap();

        if instruction.eq("forward") {
            x += quantity;
            depth += aim * quantity;
        } else if instruction.eq("up") {
            aim -= quantity;
        } else if instruction.eq("down") {
            aim += quantity;
        }
    }
    return x * depth;
}
