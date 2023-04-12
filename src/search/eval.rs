use cozy_chess::{Board, Color, Piece};
use once_cell::sync::Lazy;
use super::evaluation::piece_square_table::{*};

pub const MG_PIECE_VALUES: [i16; 6] = [82, 337, 365, 477, 1025, 0];
pub const EG_PIECE_VALUES: [i16; 6] = [94, 281, 297, 512, 936, 0];
pub const PHASE_VALUES: [i16; 6] = [0, 1, 1, 2, 4, 0];
static PST: Lazy<PieceSquareTable> = Lazy::new(PieceSquareTable::new);

pub fn eval(board: &Board) -> i16 {
    let mut score = 0;

    let mut mg = 0;
    let mut eg = 0;
    let mut game_phase = 0;

    for square in board.occupied() {
        let piece = board.piece_on(square).unwrap() as usize;
        let color = board.color_on(square).unwrap() as usize;
        let sq = square as usize;

        game_phase += PHASE_VALUES[piece];
        mg += PST.mg_table[color + piece * 2][sq];
        eg += PST.eg_table[color + piece * 2][sq];
    }

    let mg_weight = game_phase.min(24);
    let eg_weight = 24 - mg_weight;

    score += (((mg * mg_weight) + (eg * eg_weight)) / 24) as i16;

    match board.side_to_move() {
        Color::White => score,
        Color::Black => -score,
    }
}

#[cfg(test)]
mod test {
    use super::eval;
    use cozy_chess::Board;

    #[test]
    fn eval_sanity() {
        let board = Board::from_fen(
            "rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            false,
        )
            .unwrap();

        assert!(eval(&board) > 0)
    }
}
