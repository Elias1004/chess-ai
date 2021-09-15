use rand::prelude::*;
use rayon::prelude::*;

use crate::{board::*, figures::*};

// Positive: Good for white, negative: Good for black
pub fn total_score(board: &Board) -> Score {
    board
        .0
        .iter()
        .flatten()
        .map(|cell| cell.map_or(0, Piece::score))
        .sum()
}

pub fn get_all_possible_moves(board: &Board, whose_turn: Color) -> Vec<Move> {
    let mut result = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            let from = (x, y);
            match board.get(from) {
                Some(piece) if piece.color == whose_turn => {
                    piece.reachables(from, board, |pos| {
                        result.push(Move { from, to: pos })
                    });
                }
                _ => {}
            }
        }
    }
    result
}

pub fn calculate_best_move(
    board: &Board,
    whose_turn: Color,
    max_look_ahead: usize,
) -> Option<Move> {
    let moves = get_all_possible_moves(board, whose_turn);
    let scores: Vec<Score> = moves
        .par_iter()
        .map(|&mov| {
            let score =
                calculate_future_score(board, mov, max_look_ahead, whose_turn);
            score * whose_turn.as_number() as Score
        })
        .collect();
    let max_score = scores.iter().max()?;
    // From the indices of values wich are all equal to the max, choose a random one
    let index = scores
        .iter()
        .enumerate()
        .filter(|&(_, score)| score == max_score)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .0;
    Some(moves[index])
    //.expect("Can't find move to make (AI was checkmated)")
}

fn calculate_future_score(
    board: &Board,
    mov: Move,
    look_ahead: usize,
    whose_turn: Color,
) -> Score {
    let mut copy = board.clone();
    debug_assert_eq!(
        copy.get(mov.from).map(|piece| piece.color),
        Some(whose_turn)
    );
    debug_assert_ne!(
        copy.get(mov.to).map(|piece| piece.color),
        Some(whose_turn)
    );
    copy.do_move(mov);
    if look_ahead == 0 {
        return total_score(&copy);
    }
    // Now calculate the scores for all the moves the opponent can do
    let moves = get_all_possible_moves(&copy, whose_turn.flipped());
    let scores = moves.iter().map(|&enemy_mov| {
        calculate_future_score(
            &copy,
            enemy_mov,
            look_ahead - 1,
            whose_turn.flipped(),
        )
    });
    match whose_turn.flipped() {
        // The enemy chooses the the best score for itself
        Color::White => scores.max().unwrap_or(-10_000),
        Color::Black => scores.min().unwrap_or(10_000),
    }
}

/*
fn random_max_by_key<'a, T: Copy + 'a, U: PartialEq + Ord + std::fmt::Debug>(
    iter: impl Iterator<Item = &'a T> + Clone,
    mut key: impl FnMut(&T) -> U,
    rng: &mut impl Rng,
) -> Option<T> {
    // As iter is only iterating over references cloning it is probably cheap
    let time = std::time::Instant::now();
    let max_key: U = iter.clone().copied().map(|el| key(&el)).max()?;
    dbg!(time.elapsed());
    let time = std::time::Instant::now();
    let x = iter.filter(|el| key(*el) == max_key).choose(rng).copied();
    dbg!(time.elapsed());
    x
}
*/

//fn random_min_by_key<'a, T: Copy + Ord + 'a, U: PartialEq + Ord>(
//    iter: impl Iterator<Item = &'a T> + Clone,
//    mut key: impl FnMut(&T) -> U,
//    rng: &mut impl Rng,
//) -> Option<T> {
//    random_max_by_key(iter, |x| std::cmp::Reverse(key(x)), rng)
//}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_score() {
        assert_eq!(total_score(&Board::new()), 0);
    }

    #[bench]
    fn new_field(b: &mut test::Bencher) {
        let board: Board = Board::new();
        b.iter(|| {
            let mov = calculate_best_move(&board, Color::White, 3);
            assert!(mov.is_some());
            test::black_box(mov);
            //panic!("{:?}", board);
        });
    }
}
