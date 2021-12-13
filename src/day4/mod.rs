use std::collections::HashMap;

struct BoardNumber {
    number: usize,
    row: usize,
    column: usize,
    checked: bool,
}

fn mark_number(boards: &Vec<Vec<Vec<BoardNumber>>>, num: &usize) {
    for board in boards {
        for row in board {
            for number in row {
                if number.number == *num {
                    number.checked = true;
                }
            }
        }
    }
}

fn check_board(board: &HashMap<usize, BoardNumber>) -> bool {
    return true;
}

pub fn run1() -> usize {
    let lines = super::utils::read_file("src/day4/sample.txt", true);

    // First lines contain the numbers in order
    let numbers: Vec<usize> = lines[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", numbers);

    let mut boards: &Vec<Vec<Vec<BoardNumber>>> = &Vec::new();

    let mut actualBoard: Vec<Vec<BoardNumber>> = Vec::new();

    for (r, line) in lines.iter().enumerate() {
        if r > 1 && line.len() == 0 {
            boards.push(actualBoard);
            actualBoard = Vec::new();
        } else if r > 1 {
            let numbers = line
                .split(" ")
                .filter(|&x| x.len() > 0)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut row: Vec<BoardNumber> = Vec::new();
            for (c, num) in numbers.iter().enumerate() {
                let boardNum = BoardNumber {
                    number: *num,
                    row: r,
                    column: c,
                    checked: false,
                };
                row.push(boardNum);
            }
            actualBoard.push(row);
        }
    }

    let num: usize = 23;
    mark_number(&boards, &num);
    for board in boards.iter() {
        println!("\nNew board");
        for row in board.iter() {
            for num in row.iter() {
                print!("{}-{}", num.number, num.checked);
            }
            print!("\n");
        }
    }
    return 1;
}

pub fn run2() -> usize {
    return 1;
}
