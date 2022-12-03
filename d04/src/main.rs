mod core;

use std::process::exit;
use crate::core::*;

fn main() {
    let input = include_str!("../input.txt");
    let tables = parse_tables(input);

    let tables = tables.into_iter().map(Trans::Identity).collect();
    let winner = get_winner_brd(tables);

    if winner.is_none() {
        println!("No winner found.");
        exit(0);
    }

    let score = winner.unwrap().get_score();
    println!("{}", score);
}
