use std::{io, process::exit};

// TODO: use enum rather than num

fn main() -> io::Result<()> {
    // Board without anything -> 0
    // Player 1, 2

    //   0 1 2
    // 0 + + +
    // 1 + + +
    // 2 + + +

    // user input row:col

    let mut switch = 1;
    let mut board = [[0u8; 3]; 3];
    println!("=[Game board]>");
    print_board(board);

    loop {
        let mut input = String::new();
        println!("=[Player {}]> type row col to Tic-Tac-Toe", switch);
        io::stdin().read_line(&mut input).expect("Want str");
        let cin = input.trim();

        if cin.len() != 3 || cin.chars().nth(1) != Some(' ') {
            println!("Need proper input like 1 2 to represent (row 1, column 2)");
            continue;
        }

        // TODO: Better way to do below.
        if let [Ok(mut row), Ok(mut col)] =
            &cin.split(" ").map(|c| c.parse::<u32>()).collect::<Vec<_>>()[..]
        {
            if row > 3 || col > 3 {
                println!("Please input numbers within 3");
                continue;
            } else {
                row = row - 1;
                col = col - 1;

                // Modification of the board;
                println!("=[Player {}]> take {} {}", switch, row + 1, col + 1);

                if board[row as usize][col as usize] != 0 {
                    println!("Please don't take other player's pos");
                    continue;
                }
                
                board[row as usize][col as usize] = switch;
                print_board(board);

                let result = judge(board);
                match result {
                    0 => {}
                    1 => {
                        println!("=[Player 1] win");
                        exit(0)
                    }
                    2 => {
                        println!("=[Player 2] win");
                        exit(0)
                    }
                    _ => {}
                }

                switch = match switch {
                    1 => 2,
                    2 => 1,
                    _ => 0,
                };

                if switch == 0 {
                    println!("Switch value invalid");
                    continue;
                }
            }
        } else {
            println!("Please input numbers!");
            continue;
        }
    } // main loop
}

fn print_board(board: [[u8; 3]; 3]) {
    for row in 0..3 {
        for col in 0..3 {
            print!("{} ", board[row][col]);
        }
        println!();
    }
}

fn judge(board: [[u8; 3]; 3]) -> u8 {

    for row in 0..3 {
        if (board[row][0] == board[row][1]) && (board[row][1] == board[row][2]) {
            match board[row][0] {
                0 => {continue;}
                1 => {return 1;}
                2 => {return 2;}
                _ => {}
            }
        }
    }

    for col in 0..3 {
        if (board[0][col] == board[1][col]) && (board[1][col] == board[2][col]) {
            match board[0][col] {
                0 => {continue;}
                1 => {return 1;}
                2 => {return 2;}
                _ => {}
            }
        }
    }

    if (board[0][0] == board[1][1]) && (board[1][1] == board[2][2]) {
        match board[1][1] {
            0 => {},
            1 => {return 1;},
            2 => {return 2;},
            _ => {},
        }
    }

    if (board[0][2] == board[1][1]) && (board[1][1] == board[2][0]) {
        match board[1][1] {
            0 => {},
            1 => {return 1;},
            2 => {return 2;},
            _ => {},
        }
    }

    return 0;
}
