use crate::board::Board;
use std::cmp::min;

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
        board.empty_tiles_indices().len() as f32 / 16.
    }
}

/// Simple `BoardEvaluator` implementation which evaluation is just a normalized sum of the
/// tiles exponents
pub struct SumEvaluator;

impl BoardEvaluator for SumEvaluator {
    fn evaluate(&self, board: Board) -> f32 {
        let sum: u8 = board.into_iter().sum();
        sum as f32 / 15. * 16.
    }
}

pub struct InversionEvaluator;

impl BoardEvaluator for InversionEvaluator {
    fn evaluate(&self, board: Board) -> f32 {
        let inversion_factor =
            self.row_inversion_factor(board) + self.column_inversion_factor(board);
        // 240 = 16 * 15 is the max number of inversions
        1. - inversion_factor as f32 / 240.
    }
}

impl InversionEvaluator {
    fn row_inversion_factor(&self, board: Board) -> u8 {
        let mut inversion_factor = 0;
        for row in 0..4 {
            let mut left_value = board.get_exponent_value(4 * row);
            let mut left_right_inversions = 0;
            let mut right_left_inversions = 0;
            for col in 1..4 {
                let v = board.get_exponent_value(4 * row + col);
                if v < left_value {
                    left_right_inversions += left_value - v;
                } else if v > left_value {
                    right_left_inversions += v - left_value;
                }
                left_value = v;
            }
            inversion_factor += min(left_right_inversions, right_left_inversions);
        }
        inversion_factor
    }

    fn column_inversion_factor(&self, board: Board) -> u8 {
        let mut inversion_factor = 0;
        for col in 0..4 {
            let mut up_value = board.get_exponent_value(col);
            let mut up_bottom_inversions = 0;
            let mut bottom_up_inversions = 0;
            for row in 1..4 {
                let v = board.get_exponent_value(4 * row + col);
                if v < up_value {
                    up_bottom_inversions += up_value - v;
                } else if v > up_value {
                    bottom_up_inversions += v - up_value;
                }
                up_value = v;
            }
            inversion_factor += min(up_bottom_inversions, bottom_up_inversions);
        }
        inversion_factor
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

    #[test]
    fn test_inversion_evaluator() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            2, 4, 2, 4,
            8, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64,
        ];
        let board = Board::from(vec_board);
        let evaluator = InversionEvaluator {};

        // When
        let row_inversions = evaluator.row_inversion_factor(board);
        let col_inversions = evaluator.column_inversion_factor(board);
        assert_eq!(19, row_inversions);
        assert_eq!(24, col_inversions);
        let evaluation = evaluator.evaluate(board);
        assert_eq!(1. - (24. + 19.) / (15. * 16.), evaluation);
    }
}
