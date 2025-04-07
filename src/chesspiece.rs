use std::cmp::{max, min};

use crate::chessboard::{self, ChessBoard, ChessSquare, MoveError, DOWN, UP};

#[derive(Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
    pub first_move: bool,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self {
            kind,
            color,
            first_move: true,
        }
    }
    pub fn value(&self) -> i8 {
        match self.kind {
            PieceKind::Pawn => 1,
            PieceKind::Knight => 3,
            PieceKind::Bishop => 3,
            PieceKind::Rook => 5,
            PieceKind::Queen => 9,
            _ => 0
        }
    }

    pub fn valid_move(self, board: ChessBoard, square_1: ChessSquare, square_2: ChessSquare) -> Result<Self, MoveError> {
        // Check if out of bounds
        if let Some(_) = square_1.iter().chain(square_2.iter()).find_map(|&i| chessboard::in_bounds(i).err()) {
            return Err(MoveError::OutOfBounds)
        }

        let x_dif = max(square_1[1], square_2[1]) - min(square_1[1], square_2[1]);
        let y_dif = max(square_1[0], square_2[0]) - min(square_1[0], square_2[0]);

        let attacking = board[square_2[0]][square_2[1]].is_some();

        // Move different depending on chesspiece type
        match self.kind {
            PieceKind::Pawn => { // TODO: En passent
                let up_or_down: isize;

                match self.color {
                    PieceColor::White => up_or_down = UP,
                    PieceColor::Black => up_or_down = DOWN,
                }

                // See which way the pawn is moving
                let y_dif_direction: isize = square_2[0] as isize - square_1[0] as isize;
                let single_step = y_dif_direction == up_or_down;
                let double_step = y_dif_direction == up_or_down * 2;
                if !single_step && (!self.first_move || !double_step) {
                    return Err(MoveError::InvalidMove);
                }

                // Check for staying in file, except when attacking
                if attacking && 1 == x_dif && single_step {
                    return Err(MoveError::InvalidMove);
                } else if square_1[1] != square_2[1] {
                    return Err(MoveError::InvalidMove);
                }

                Ok(Self{kind: self.kind, color: self.color, first_move: false})
            },
            PieceKind::Knight => {
                if min(x_dif, y_dif) != 1 ||
                   max(x_dif, y_dif) != 2 {
                    return Err(MoveError::InvalidMove);
                }
                Ok(Self{kind: self.kind, color: self.color, first_move: false})
            },
            PieceKind::Bishop => {
                if x_dif != y_dif {
                    return Err(MoveError::InvalidMove);
                }
                Ok(Self{kind: self.kind, color: self.color, first_move: false})
            },
            PieceKind::Rook => {
                if min(x_dif, y_dif) != 0 {
                    return Err(MoveError::InvalidMove);
                }
                Ok(Self{kind: self.kind, color: self.color, first_move: false})
            },
            PieceKind::Queen => {
                if x_dif != y_dif && min(x_dif, y_dif) != 0 {
                    return Err(MoveError::InvalidMove);
                }
                Ok(Self{kind: self.kind, color: self.color, first_move: false})
            },
            PieceKind::King => {
                if x_dif > 1 || y_dif > 1 {
                    return Err(MoveError::InvalidMove);
                }
                Ok(Self{kind: self.kind, color: self.color, first_move: false})
            },
        }
    }
}