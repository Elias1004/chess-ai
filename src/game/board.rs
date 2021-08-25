use std::fmt;

use super::figures::*;

pub struct Board {
    buf: Box<[[Piece; 8]; 8]>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            buf: Box::new([[EMPTY; 8]; 8]),
        }
    }

    pub fn new() -> Self {
        let mut buf = Box::new([
            [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK],
            [PAWN, PAWN, PAWN, PAWN, PAWN, PAWN, PAWN, PAWN],
            [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [PAWN, PAWN, PAWN, PAWN, PAWN, PAWN, PAWN, PAWN],
            [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK],
        ]);

        for i in 0..8 {
            buf[0][i] = buf[0][i] | BLACK;
            buf[1][i] = buf[1][i] | BLACK;
        }

        Self { buf }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.buf.iter().enumerate() {
            write!(f, "{:?}", row)?;
            if i != self.buf.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[test]
fn print() {
    let a = Board::new();
    let expected = "\
[B ROOK  , B KNIGHT, B BISHOP, B QUEEN , B KING  , B BISHOP, B KNIGHT, B ROOK  ]
[B PAWN  , B PAWN  , B PAWN  , B PAWN  , B PAWN  , B PAWN  , B PAWN  , B PAWN  ]
[E       , E       , E       , E       , E       , E       , E       , E       ]
[E       , E       , E       , E       , E       , E       , E       , E       ]
[E       , E       , E       , E       , E       , E       , E       , E       ]
[E       , E       , E       , E       , E       , E       , E       , E       ]
[W PAWN  , W PAWN  , W PAWN  , W PAWN  , W PAWN  , W PAWN  , W PAWN  , W PAWN  ]
[W ROOK  , W KNIGHT, W BISHOP, W QUEEN , W KING  , W BISHOP, W KNIGHT, W ROOK  ]";
    assert_eq!(format!("{:?}", a), expected);
}
