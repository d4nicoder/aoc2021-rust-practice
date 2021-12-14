use std::collections::HashMap;
use std::ops::Deref;

struct BoardNumber {
    board: usize,
    number: usize,
    row: usize,
    column: usize,
    checked: bool,
}

fn mark_number(boards: &mut Vec<BoardNumber>, num: usize) {
    println!("  - marking ({}) with num {}", boards.len(), num);
    for i in 0..boards.len() {
        if boards.get(i).unwrap().number == num {
            println!("    - FOUND!");
            boards.get_mut(i).unwrap().checked = true;
            if check_board(boards, i) {
                println!("    - BOARD {} IS SOLVED!", boards.get(i).unwrap().board);
            }
            return;
        }
    }
}

fn check_board(boards: &Vec<BoardNumber>, idx: usize) -> bool {
    for i in 0..boards.len() {
        println!("Board: {}", i);
    }

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

    let mut boards: Vec<BoardNumber> = Vec::new();
    let mut board_index: usize = 0;

    for (r, line) in lines.iter().enumerate() {
        if r > 1 && line.len() == 0 {
            board_index += 1;
        } else if r > 1 {
            let numbers = line
                .split(" ")
                .filter(|&x| x.len() > 0)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for (c, num) in numbers.iter().enumerate() {
                let board_num = BoardNumber {
                    board: board_index,
                    number: *num,
                    row: r,
                    column: c,
                    checked: false,
                };
                boards.push(board_num);
            }
        }
    }
    let mut marked_boards: Vec<BoardNumber> = boards;
    for num in numbers {
        println!("Marking number: {}", num);
        mark_number(&mut marked_boards, num);
    }

    println!("markedBoards size: {}", marked_boards.len());
    for board in marked_boards.iter() {
        println!(
            "Board {}, row {}, column {}, number {}, checked {}",
            board.board, board.row, board.column, board.number, board.checked
        );
    }
    return 1;
}

pub fn run2() -> usize {
    return 1;
}
