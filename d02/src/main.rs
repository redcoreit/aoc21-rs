use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// forward 5
// down 5
// forward 8
// up 3
// down 8
// forward 2

struct MovementData {
    vertical: i32,
    horizontal: i32,
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let path = Path::new(&cwd).join("input.txt");
    let file_content = File::open(path).unwrap();
    let lines = BufReader::new(file_content).lines();
    let mut moves = MovementData { vertical: 0, horizontal: 0 };

    for line in lines {
        let current_move = parse_line(&line.unwrap());
        moves.vertical += current_move.vertical;
        moves.horizontal += current_move.horizontal;
    }

    println!("Horizontal: {}", moves.horizontal);
    println!("Vertical: {}", moves.vertical);
    println!("Total: {}", moves.vertical * moves.horizontal);
}

fn parse_line(line: &str) -> MovementData {
    let terms : Vec<&str> = line.split(' ').collect();

    let num :i32 = terms[1].parse().expect("Number expected.");

    let result = match terms[0] {
        "forward" => MovementData { horizontal: num, vertical: 0 },
        "up" => MovementData { horizontal: 0, vertical: -num },
        "down" => MovementData { horizontal: 0, vertical: num },
        _ => panic!("Invalid command found."),
    };

    result
}
