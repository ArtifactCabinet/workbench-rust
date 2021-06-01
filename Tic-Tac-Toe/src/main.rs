use std::fmt::{self, Display, Formatter};
use std::io;
use std::process::exit;
// TODO: use enum rather than num

#[derive(Copy, Clone, PartialEq, Eq)]
//       ^     ^ Rust quirk
enum PosType {
    P1,
    P2,
    None,
}

impl Display for PosType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PosType::P1 => write!(f, "Player 1"),
            PosType::P2 => write!(f, "Player 2"),
            _ => todo!(), // This line should not be reached
        }
    }
}

fn main() -> io::Result<()> {
    // Board without anything -> 0
    // Player 1, 2

    //   0 1 2
    // 0 + + +
    // 1 + + +
    // 2 + + +

    // user input row:col

    let mut switch = PosType::P1;
    let mut board = [[PosType::None; 3]; 3];
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
                println!("= {} > take {} {}", switch, row + 1, col + 1);

                if board[row as usize][col as usize] != PosType::None {
                    println!("Please don't take other player's pos");
                    continue;
                }

                board[row as usize][col as usize] = switch;
                print_board(board);

                let result = judge(board);
                match result {
                    PosType::None => {}
                    PosType::P1 => {
                        println!("==[Player 1] win");
                        exit(0)
                    }
                    PosType::P2 => {
                        println!("=[Player 2] win");
                        exit(0)
                    }
                }

                switch = match switch {
                    PosType::P1 => PosType::P2,
                    PosType::P2 => PosType::P1,
                    PosType::None => {
                        println!("Switch value invalid");
                        continue;
                    }
                };
            }
        } else {
            println!("Please input numbers!");
            continue;
        }
    } // main loop
}

fn print_board(board: [[PosType; 3]; 3]) {
    for row in 0..3 {
        for col in 0..3 {
            match board[row][col] {
                PosType::None => {
                    print!("{} ", 0)
                }
                PosType::P1 => {
                    print!("{} ", 1)
                }
                PosType::P2 => {
                    print!("{} ", 2)
                }
            }
        }
        println!();
    }
}

// TODO: There should be a better way to handle the thing below
// NOTE: enum type is werid
fn judge(board: [[PosType; 3]; 3]) -> PosType {
    for row in 0..3 {
        match (board[row][0], board[row][1], board[row][2]) {
            (PosType::None, PosType::None, PosType::None) => {
                continue;
            }
            (PosType::P1, PosType::P1, PosType::P1) => {
                return PosType::P1;
            }
            (PosType::P2, PosType::P2, PosType::P2) => {
                return PosType::P2;
            }
            _ => {}
        }
    }

    for col in 0..3 {
        match (board[0][col], board[1][col], board[2][col]) {
            (PosType::None, PosType::None, PosType::None) => {
                continue;
            }
            (PosType::P1, PosType::P1, PosType::P1) => {
                return PosType::P1;
            }
            (PosType::P2, PosType::P2, PosType::P2) => {
                return PosType::P2;
            }
            _ => {}
        }
    }

    match (board[0][0], board[1][1], board[2][2]) {
        (PosType::None, PosType::None, PosType::None) => {}
        (PosType::P1, PosType::P1, PosType::P1) => {
            return PosType::P1;
        }
        (PosType::P2, PosType::P2, PosType::P2) => {
            return PosType::P2;
        }
        _ => {}
    }

    match (board[0][2], board[1][1], board[2][0]) {
        (PosType::None, PosType::None, PosType::None) => {}
        (PosType::P1, PosType::P1, PosType::P1) => {
            return PosType::P1;
        }
        (PosType::P2, PosType::P2, PosType::P2) => {
            return PosType::P2;
        }
        _ => {}
    }

    return PosType::None;
}
