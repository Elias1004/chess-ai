use serde::{Deserialize, Serialize};

use super::figures::*;

pub type Pos = (i8, i8); // Allow negatives to avoid underflow for values outside the board

pub type Score = i32;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Board(pub [[Option<Piece>; 8]; 8]);

impl Board {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self([[None; 8]; 8])
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        DEFAULT_FIELD
    }

    pub fn get(&self, pos: Pos) -> Option<Piece> {
        let (x, y) = pos;
        self.0[x as usize][y as usize]
    }

    pub fn do_move(&mut self, mov: Move) {
        debug_assert!(self.get(mov.from).is_some(), "Bad move: {:?}", mov);
        let mut figure = self.0[mov.from.0 as usize][mov.from.1 as usize]
            .take()
            .unwrap(); // Unwrapping fails if the `from` position was empty
        let color = figure.color;
        // Transfrom pawns on the to queens
        if figure.figure == Figure::Pawn {
            if color == Color::White && mov.to.1 == 0 {
                figure = Piece {
                    color,
                    figure: Figure::Queen,
                };
            } else if color == Color::Black && mov.to.1 == 7 {
                figure = Piece {
                    color,
                    figure: Figure::Queen,
                };
            }
        }
        self.0[mov.to.0 as usize][mov.to.1 as usize] = Some(figure);
    }

    pub fn is_inside(&self, pos: Pos) -> bool {
        let (x, y) = pos;
        x >= 0 && x < 8 && y >= 0 && y < 8
    }
}

use Color::*;
use Figure::*;

#[allow(unused)]
#[rustfmt::skip]
const DEFAULT_FIELD: Board = Board([[Some(Piece { color: Black, figure: Rook }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Rook })], [Some(Piece { color: Black, figure: Knight }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Knight })], [Some(Piece { color: Black, figure: Bishop }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Bishop })], [Some(Piece { color: Black, figure: Queen }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Queen })], [Some(Piece { color: Black, figure: King }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: King })], [Some(Piece { color: Black, figure: Bishop }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Bishop })], [Some(Piece { color: Black, figure: Knight }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Knight })], [Some(Piece { color: Black, figure: Rook }), Some(Piece { color: Black, figure: Pawn }), None, None, None, None, Some(Piece { color: White, figure: Pawn }), Some(Piece { color: White, figure: Rook })]])
;
