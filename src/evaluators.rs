use crate::board::Board;

/// Evaluate a `Board` by mapping it to a number. The higher the number, the better the board
/// state.
pub trait BoardEvaluator {
    fn evaluate(&self, board: Board) -> f32;
}

/// A simple implementation of `BoardEvaluator` which evaluates a board by simply computing
/// the proportion of tiles which are zero.
pub struct ZeroCountEvaluator;

impl BoardEvaluator for ZeroCountEvaluator {
    fn evaluate(&self, board: Board) -> f32 {
        let zeros: f32 = Vec::from(board)
            .into_iter()
            .map(|tile| if tile == 0 { 1. } else { 0. })
            .sum();
        zeros / 16.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_count_evaluator() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            0, 2, 0, 0,
            0, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);
        let evaluator = ZeroCountEvaluator {};

        // When
        let evaluation = evaluator.evaluate(board);
        assert_eq!(7. / 16., evaluation);
    }
}
