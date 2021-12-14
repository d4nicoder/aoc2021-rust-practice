use std::collections::HashMap;

struct BoardNumber {
    board: usize,
    number: usize,
    row: usize,
    column: usize,
    checked: bool,
}

fn mark_number(boards: &Vec<BoardNumber>, num: usize) {
    println!("  - marking ({}) with num {}", boards.len(), num);
    for (i, number) in boards.iter().enumerate() {
        if number.number == num {
            println!("    - FOUND!");
            let mut num = boards.get(i).as_mut().unwrap();
            num.checked = true;
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

    let mut boards: Vec<BoardNumber> = Vec::new();
    let mut boardIndex: usize = 0;

    for (r, line) in lines.iter().enumerate() {
        if r > 1 && line.len() == 0 {
            boardIndex += 1;
        } else if r > 1 {
            let numbers = line
                .split(" ")
                .filter(|&x| x.len() > 0)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut row: Vec<BoardNumber> = Vec::new();
            for (c, num) in numbers.iter().enumerate() {
                let boardNum = BoardNumber {
                    board: boardIndex,
                    number: *num,
                    row: r,
                    column: c,
                    checked: false,
                };
                boards.push(boardNum);
            }
        }
    }
    let mut markedBoards: Vec<BoardNumber> = boards;
    for num in numbers {
        println!("Marking number: {}", num);
        mark_number(&markedBoards, num);
    }

    println!("markedBoards size: {}", markedBoards.len());
    for board in markedBoards.iter() {
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
