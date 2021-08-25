use std::{
    fmt,
    ops::{BitAnd, BitOr},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece(u8);

pub const PAWN: Piece = Piece(1);
pub const BISHOP: Piece = Piece(2);
pub const KNIGHT: Piece = Piece(3);
pub const ROOK: Piece = Piece(4);
pub const QUEEN: Piece = Piece(5);
pub const KING: Piece = Piece(6);

pub const EMPTY: Piece = Piece(0x0);
pub const BLACK: Piece = Piece(0x10);

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "E       ");
        }
        if self.is_white() {
            write!(f, "W ")?;
        }
        if self.is_black() {
            write!(f, "B ")?;
        }
        if self.is_type(PAWN) {
            return write!(f, "PAWN  ");
        }
        if self.is_type(BISHOP) {
            return write!(f, "BISHOP");
        }
        if self.is_type(KNIGHT) {
            return write!(f, "KNIGHT");
        }
        if self.is_type(ROOK) {
            return write!(f, "ROOK  ");
        }
        if self.is_type(QUEEN) {
            return write!(f, "QUEEN ");
        }
        if self.is_type(KING) {
            return write!(f, "KING  ");
        }
        Ok(())
    }
}

impl BitOr for Piece {
    type Output = Piece;

    fn bitor(self, rhs: Self) -> Self::Output {
        Piece(self.0 | rhs.0)
    }
}

impl BitAnd for Piece {
    type Output = Piece;

    fn bitand(self, rhs: Self) -> Self::Output {
        Piece(self.0 & rhs.0)
    }
}

impl Piece {
    pub fn is_empty(self) -> bool {
        self == EMPTY
    }

    pub fn is_white(self) -> bool {
        self & BLACK == Piece(0)
    }

    pub fn is_black(self) -> bool {
        self & BLACK != Piece(0)
    }

    pub fn is_type(self, other: Self) -> bool {
        // Last 4 bits are equals
        (self.0 & 0x0f) == (other.0 & 0x0f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pieces() {
        let a = EMPTY;
        assert!(a.is_empty());
        assert!(!a.is_type(QUEEN));
        assert!(!a.is_type(ROOK));

        let a = PAWN;
        assert!(!a.is_empty());
        assert!(a.is_white());
        assert!(!a.is_black());
        assert!(a.is_type(PAWN));

        let a = PAWN | BLACK;
        assert!(!a.is_empty());
        assert!(a.is_black());
        assert!(!a.is_white());
        assert!(a.is_type(PAWN));

        let a = KING;
        assert!(!a.is_black());
        assert!(a.is_white());
        assert!(a.is_type(KING));

        let a = QUEEN | BLACK;
        assert!(a.is_black());
        assert!(!a.is_white());
        assert!(a.is_type(QUEEN));
    }
}
