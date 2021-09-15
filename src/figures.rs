use serde::{Deserialize, Serialize};

use crate::board::{Board, Pos, Score};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub color: Color,
    pub figure: Figure,
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, Debug, Deserialize)]
pub enum Color {
    White = 1,
    Black = -1,
}

impl Color {
    pub fn as_number(self) -> i8 {
        self as i8
    }

    pub fn flipped(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Serialize, Copy, Clone, PartialEq, Eq, Debug, Deserialize)]
pub enum Figure {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn reachables(
        self,
        pos: Pos,
        board: &Board,
        mut insert_into: impl FnMut(Pos),
    ) {
        macro_rules! straight_lines {
            ($insert_into:ident, $( ($x:expr, $y:expr) ),*) => {{
                $(
                    let (mut x, mut y) = pos;
                    loop {
                        x += $x;
                        y += $y;
                        if !board.is_inside((x, y)) {
                            break;
                        }
                        let cell = board.get((x, y));
                        match cell {
                            Some(Piece { color, .. }) if color == self.color => {
                                // Stop at allies
                                break;
                            }
                            Some(Piece { color, .. }) if color != self.color => {
                                // Stop at enemy, but hit them
                                $insert_into((x, y));
                                break;
                            }
                            _ => ()
                        }
                        $insert_into((x, y));
                    }
                )*
            }};
        }

        match self.figure {
            Figure::Rook => {
                straight_lines!(insert_into, (1, 0), (-1, 0), (0, 1), (0, -1))
            }
            Figure::Bishop => {
                straight_lines!(insert_into, (1, 1), (1, -1), (-1, 1), (-1, -1))
            }
            Figure::Queen => {
                straight_lines!(
                    insert_into,
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1)
                )
            }
            Figure::King => {
                for (x, y) in [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ] {
                    let target = (x + pos.0, y + pos.1);
                    if !board.is_inside(target) {
                        break;
                    }
                    match board.get(target) {
                        Some(Piece { color, .. }) if color == self.color => {
                            // Allied figure, can't move here
                        }
                        _ => {
                            // Empty or Enemy figure
                            insert_into(target);
                        }
                    }
                }
            }
            Figure::Knight => {
                for (x, y) in [
                    (1, 2),
                    (-1, 2),
                    (1, -2),
                    (-1, -2),
                    (2, 1),
                    (-2, 1),
                    (2, -1),
                    (-2, -1),
                ] {
                    let target = (x + pos.0, y + pos.1);
                    if !board.is_inside(target) {
                        break;
                    }
                    match board.get(target) {
                        Some(Piece { color, .. }) if color == self.color => {
                            // Allied figure
                        }
                        _ => {
                            // Empty or Enemy figure
                            insert_into(target);
                        }
                    }
                }
            }
            Figure::Pawn => {
                match self.color {
                    Color::White => {
                        // Forwards
                        let target = (pos.0 + 0, pos.1 - 1);
                        if board.is_inside(target) {
                            match board.get(target) {
                                None => {
                                    // Empty field
                                    insert_into(target);
                                    // Forwards double
                                    if pos.1 == 6 {
                                        let target = (pos.0 + 0, pos.1 - 2);
                                        if board.is_inside(target) {
                                            match board.get(target) {
                                                None => {
                                                    // Empty field
                                                    insert_into(target);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        // Left hit
                        let target = (pos.0 - 1, pos.1 - 1);
                        if board.is_inside(target) {
                            match board.get(target) {
                                Some(Piece { color, .. })
                                    if color != self.color =>
                                {
                                    // Enemy figure
                                    insert_into(target);
                                }
                                _ => {}
                            }
                        }
                        // Right hit
                        let target = (pos.0 + 1, pos.1 - 1);
                        if board.is_inside(target) {
                            match board.get(target) {
                                Some(Piece { color, .. })
                                    if color != self.color =>
                                {
                                    //Enemy figure
                                    insert_into(target);
                                }
                                _ => {}
                            }
                        }
                    }
                    Color::Black => {
                        // Forwards
                        let target = (pos.0 + 0, pos.1 + 1);
                        if board.is_inside(target) {
                            match board.get(target) {
                                None => {
                                    // Empty field
                                    insert_into(target);
                                    // Forwards double
                                    if pos.1 == 1 {
                                        let target = (pos.0 + 0, pos.1 + 2);
                                        if board.is_inside(target) {
                                            match board.get(target) {
                                                None => {
                                                    // Empty field
                                                    insert_into(target);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        // Left hit
                        let target = (pos.0 - 1, pos.1 + 1);
                        if board.is_inside(target) {
                            match board.get(target) {
                                Some(Piece { color, .. })
                                    if color != self.color =>
                                {
                                    //Enemy figure
                                    insert_into(target);
                                }
                                _ => {}
                            }
                        }
                        // Right hit
                        let target = (pos.0 + 1, pos.1 + 1);
                        if board.is_inside(target) {
                            match board.get(target) {
                                Some(Piece { color, .. })
                                    if color != self.color =>
                                {
                                    // Enemy figure
                                    insert_into(target);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn reachables_collect(self, pos: Pos, board: &Board) -> Vec<Pos> {
        let mut result = Vec::new();
        self.reachables(pos, board, |pos| result.push(pos));
        result
    }

    pub fn score(self) -> Score {
        use Figure::*;
        let score = match self.figure {
            Pawn => 1,
            Bishop => 3,
            Knight => 3,
            Rook => 5,
            Queen => 9,
            King => 1000,
        };
        score * self.color.as_number() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rook() {
        let mut board = Board::empty();
        board.0[0][0] = Some(Piece {
            color: Color::White,
            figure: Figure::Rook,
        });
        let reachables =
            board.0[0][0].unwrap().reachables_collect((0, 0), &board);
        assert_eq!(
            reachables,
            vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
                (0, 7),
            ]
        );
    }

    #[test]
    fn blocked_rook() {
        let mut board = Board::empty();
        board.0[0][0] = Some(Piece {
            color: Color::White,
            figure: Figure::Rook,
        });
        board.0[5][0] = Some(Piece {
            color: Color::White,
            figure: Figure::Pawn,
        });
        board.0[0][3] = Some(Piece {
            color: Color::Black,
            figure: Figure::Pawn,
        });
        let mut reachables =
            board.0[0][0].unwrap().reachables_collect((0, 0), &board);
        reachables.sort();
        let mut expected =
            vec![(1, 0), (2, 0), (3, 0), (4, 0), (0, 1), (0, 2), (0, 3)];
        expected.sort();
        assert_eq!(reachables, expected);
    }

    #[test]
    fn white_king() {
        let mut board = Board::empty();
        board.0[4][4] = Some(Piece {
            color: Color::White,
            figure: Figure::King,
        });
        board.0[5][4] = Some(Piece {
            color: Color::White,
            figure: Figure::Pawn,
        });
        board.0[3][3] = Some(Piece {
            color: Color::Black,
            figure: Figure::Pawn,
        });
        board.0[3][5] = Some(Piece {
            color: Color::Black,
            figure: Figure::Pawn,
        });
        let mut reachables =
            board.0[4][4].unwrap().reachables_collect((4, 4), &board);
        reachables.sort();
        let mut expected =
            vec![(3, 3), (4, 3), (5, 3), (5, 5), (4, 5), (3, 5), (3, 4)];
        expected.sort();
        assert_eq!(reachables, expected);
    }

    #[test]
    fn black_king() {
        let mut board = Board::empty();
        board.0[4][4] = Some(Piece {
            color: Color::Black,
            figure: Figure::King,
        });
        board.0[5][4] = Some(Piece {
            color: Color::Black,
            figure: Figure::Pawn,
        });
        board.0[3][3] = Some(Piece {
            color: Color::White,
            figure: Figure::Pawn,
        });
        board.0[3][5] = Some(Piece {
            color: Color::White,
            figure: Figure::Pawn,
        });
        let mut reachables =
            board.0[4][4].unwrap().reachables_collect((4, 4), &board);
        reachables.sort();
        let mut expected =
            vec![(3, 3), (4, 3), (5, 3), (5, 5), (4, 5), (3, 5), (3, 4)];
        expected.sort();
        assert_eq!(reachables, expected);
    }
}
