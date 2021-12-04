use whiteread::parse_string;

// assume boards are 5x5
const BOARD_SIZE: usize = 5;

pub fn part1(input: String) -> Option<u64> {
    let mut input = input.lines();
    let numbers = input.next()?
                       .split(',')
                       .map(|s| s.parse::<u8>().unwrap());

    let mut boards: Vec<Vec<Vec<(u8, bool)>>> = Vec::new();
    while input.next().is_some() {
        let mut board = Vec::new();
        for _ in 0..BOARD_SIZE {
            let row: Vec<u8> = parse_string(input.next()?).ok()?;
            board.push(row.into_iter().map(|num| (num, false)).collect());
        }
        boards.push(board);
    }

    let mut result = 0;
    for num in numbers {
        for board in &mut boards {
            for row in board {
                row.iter_mut()
                    .filter(|(n, _)| *n == num)
                    .for_each(|(_, b)| *b = true);
            }
        }

        let winning_boards = check_winners(&boards);
        if winning_boards.len() > 0 {
            result = calculate_score(winning_boards[0].clone(), num);
            break;
        }
    }

    Some(result)
}

fn check_winners(boards: &Vec<Vec<Vec<(u8, bool)>>>) -> Vec<Vec<Vec<(u8, bool)>>> {
    let mut winning_boards: Vec<Vec<Vec<(u8, bool)>>> = Vec::new();
    for board in boards {
        let mut cols = vec![0; BOARD_SIZE];
        for row in board {
            for (i, (_, ticked)) in row.iter().enumerate() {
                if *ticked {
                    cols[i] += 1;
                }
            }
            if row.iter().all(|(_, b)| *b) {
                winning_boards.push(board.clone());
            }
        }

        if cols.into_iter().any(|n| n as usize == BOARD_SIZE) {
            winning_boards.push(board.clone());
        }
    }

    winning_boards
}

fn calculate_score(board: Vec<Vec<(u8, bool)>>, winning_num: u8) -> u64 {
    let mut sum: u64 = 0;
    board.into_iter().for_each(|row| {
        row.into_iter().for_each(|(num, ticked)| {
            if !ticked {
                sum += num as u64;
            }
        });
    });

    sum * winning_num as u64
}

pub fn part2(input: String) -> Option<u64> {
    let mut input = input.lines();
    let numbers = input.next()?
                       .split(',')
                       .map(|s| s.parse::<u8>().unwrap());

    let mut boards: Vec<Vec<Vec<(u8, bool)>>> = Vec::new();
    while input.next().is_some() {
        let mut board = Vec::new();
        for _ in 0..BOARD_SIZE {
            let row: Vec<u8> = parse_string(input.next()?).ok()?;
            board.push(row.into_iter().map(|num| (num, false)).collect());
        }
        boards.push(board);
    }

    let mut result = 0;
    for num in numbers {
        for board in &mut boards {
            for row in board {
                row.iter_mut()
                    .filter(|(n, _)| *n == num)
                    .for_each(|(_, b)| *b = true);
            }
        }

        for winner in check_winners(&boards) {
            result = calculate_score(winner.clone(), num);
            boards.retain(|x| x != &winner);
        }
    }

    Some(result)
}
