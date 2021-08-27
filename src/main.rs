#![feature(extend_one, test)]
extern crate test;

mod ai;
mod board;
mod figures;

//use std::time::Instant;

use board::{Board, Move};
use figures::Color;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(
        args.len(),
        2,
        "Expected 2 arguments: color and max_look_ahead"
    );
    let color = match args[0].to_lowercase().as_str() {
        "white" => Color::White,
        "black" => Color::Black,
        _ => panic!("First command line argument must be `black` or `white`"),
    };
    let max_look_ahead = args[1]
        .parse()
        .expect("Second command line argument must be a positve integer");
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        match serde_json::from_str::<Board>(&s) {
            Ok(board) => {
                //let time_taken = std::time::Instant::now();
                let choice = make_move(&board, color, max_look_ahead);
                //dbg!(time_taken.elapsed());
                serde_json::to_writer(std::io::stdout(), &choice).unwrap();
                println!();
            }
            Err(err) => {
                println!("Bad parsing of board: {}", err);
            }
        }
    }
}

fn make_move(
    board: &Board,
    color: Color,
    max_look_ahead: usize,
) -> Option<Move> {
    //dbg!(ai::total_score(board));
    ai::calculate_best_move(board, color, max_look_ahead)
}
