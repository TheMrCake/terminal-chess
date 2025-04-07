use crate::chesspiece;

pub const UP: isize = -1;
pub const DOWN: isize = 1;

pub const BOARDSIDELENGTH: usize = 8;

pub type ChessSquare = [usize; 2];
pub type ChessBoard = [[Option<chesspiece::Piece>; BOARDSIDELENGTH]; BOARDSIDELENGTH];

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds,
    InvalidMove,
    SameSquare,
    NoPiece,
}

pub fn make_move(board: ChessBoard, square_1: ChessSquare, square_2: ChessSquare) -> Result<ChessBoard, MoveError> {
    if square_1 == square_2 {
        return Err(MoveError::SameSquare);
    }
    if let Some(piece) = board[square_1[0]][square_1[1]] {
        piece.valid_move(board, square_1, square_2)?;
    } else {
        return Err(MoveError::NoPiece);
    }
    Ok(board)
}

pub fn in_bounds(val: usize) -> Result<(), MoveError> {
    if val > BOARDSIDELENGTH-1 {
        return Err(MoveError::OutOfBounds);
    }
    Ok(())
}

#[cfg(test)]
mod board_tests {
    use crate::{chessboard::{self, ChessBoard, ChessSquare}, chesspiece::{Piece, PieceKind, PieceColor}};

    #[test]
    fn test_valid_move_pawn() {
        let sq_1: ChessSquare = [1, 2];
        let piece = Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::Black,
            first_move: true,
        };
        let board: ChessBoard = [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(piece), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        assert!(chessboard::make_move(board, sq_1, [2, 2]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [3, 2]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [2, 3]).is_err());
        assert!(chessboard::make_move(board, sq_1, [4, 2]).is_err());
    }

    #[test]
    fn test_valid_move_knight() {
        let sq_1: ChessSquare = [1, 2];
        let piece = Piece {
            kind: PieceKind::Knight,
            color: PieceColor::Black,
            first_move: true,
        };
        let board: ChessBoard = [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(piece), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        assert!(chessboard::make_move(board, sq_1, [3, 3]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [3, 4]).is_err());
    }

    #[test]
    fn test_valid_move_bishop() {
        let sq_1: ChessSquare = [1, 2];
        let piece = Piece {
            kind: PieceKind::Bishop,
            color: PieceColor::Black,
            first_move: true,
        };
        let board: ChessBoard = [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(piece), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        assert!(chessboard::make_move(board, sq_1, [2, 3]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [3, 4]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [2, 2]).is_err());
    }

    #[test]
    fn test_valid_move_rook() {
        let sq_1: ChessSquare = [1, 2];
        let piece = Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
            first_move: true,
        };
        let board: ChessBoard = [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(piece), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        assert!(chessboard::make_move(board, sq_1, [2, 2]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [3, 2]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [2, 3]).is_err());
    }

    #[test]
    fn test_valid_move_queen() {
        let sq_1: ChessSquare = [1, 2];
        let piece = Piece {
            kind: PieceKind::Queen,
            color: PieceColor::Black,
            first_move: true,
        };
        let board: ChessBoard = [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(piece), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        assert!(chessboard::make_move(board, sq_1, [2, 3]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [3, 4]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [2, 2]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [2, 5]).is_err());
    }

    #[test]
    fn test_valid_move_king() {
        let sq_1: ChessSquare = [1, 2];
        let piece = Piece {
            kind: PieceKind::King,
            color: PieceColor::Black,
            first_move: true,
        };
        let board: ChessBoard = [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(piece), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        assert!(chessboard::make_move(board, sq_1, [2, 3]).is_ok());
        assert!(chessboard::make_move(board, sq_1, [3, 4]).is_err());
        assert!(chessboard::make_move(board, sq_1, [2, 2]).is_ok());
    }
}