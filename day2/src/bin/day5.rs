use std::str::SplitWhitespace;

fn main() {
    let data = "    [D]....
[N] [C]....
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    //let day = 0;
      let data = aoc::get_aoc_file(5);

    parse_things_part1(&data);
    parse_things_part2(&data);
}

fn create_board(rows: &str) -> Vec<Vec<char>> {
    let first_line = rows.chars().take_while(|&chr| chr != '\n').collect::<String>();
    let mut board: Vec<Vec<char>> = Vec::new();
    println!("{:?} - {:?}", first_line, first_line.len());
    for _ in 0..1 + first_line.len() / 4 {
        let mut column = Vec::new();
        board.push(column)
    }
    println!("board created with {:?} columns", board.len());
    {
        for row in rows.split("\n") {
            if row.is_empty() || row.trim().starts_with("1") {
                for i in 0..board.len() {
                    board[i].reverse()
                }
                return board;
            }

            let mut bi = 0;
            for (i, value) in row.chars().enumerate() {
                println!("index {i}, value: {value} row: {row}");
                if i % 4 == 1 {
                    println!("adding..");
                    if value != '.' && value != ' ' {
                        board[bi].push(value);
                    }
                    bi += 1;
                }
            }
        }
    }
    return board;
}


fn parse_things_part2(data: &str) {
    let mut board = create_board(data);
    println!("board to work with: {:?}", board);

    for row in data.split("\n") {
        if row.starts_with("move") {
            // move 1 from 2 to 1
            let mut iter = row.split_whitespace().collect::<Vec<&str>>();
            let times = iter[1].parse::<usize>().unwrap();
            let mut buffer: Vec<char> = Vec::new();
            let to = iter[5].parse::<usize>().unwrap();
            for _ in 0..times {
                let from = iter[3].parse::<usize>().unwrap();
                let value_to_move = board[from - 1].pop().unwrap();
                buffer.push(value_to_move);
                println!("board: {:?}", board);
            }
            buffer.reverse();
            board[to - 1].append(&mut buffer);
        }
    }
    println!("board worked with: {:?}", board);
    for i in 0..board.len() {
        print!("{}", board[i].last().unwrap())
    }
    println!()
}

fn parse_things_part1(data: &str) {
    let mut board = create_board(data);
    println!("board to work with: {:?}", board);

    for row in data.split("\n") {
        if row.starts_with("move") {
            // move 1 from 2 to 1
            let mut iter = row.split_whitespace().collect::<Vec<&str>>();
            let times = iter[1].parse::<usize>().unwrap();
            for _ in 0..times {
                let from = iter[3].parse::<usize>().unwrap();
                let to = iter[5].parse::<usize>().unwrap();
                let value_to_move = board[from - 1].pop().unwrap();
                board[to - 1].push(value_to_move);
                println!("board: {:?}", board);
            }
        }
    }


    println!("board worked with: {:?}", board);
    for i in 0..board.len() {
        print!("{}", board[i].last().unwrap())
    }
    println!()
}
