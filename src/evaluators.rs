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
        let zeros: f32 = Vec::from(board)
            .into_iter()
            .map(|tile| if tile == 0 { 1. } else { 0. })
            .sum();
        zeros / 16.
    }
}

pub struct InversionEvaluator;

impl BoardEvaluator for InversionEvaluator {
    fn evaluate(&self, board: Board) -> f32 {
        let inversions = self.row_inversions(board) + self.column_inversions(board);
        // 16 is the max number of inversions
        1. - inversions as f32 / 16.
    }
}

impl InversionEvaluator {
    fn row_inversions(&self, board: Board) -> usize {
        let mut inversions = 0;
        for row in 0..4 {
            let mut left_value = board.get_value(4 * row);
            let mut left_right_inversions = 0;
            let mut right_left_inversions = 0;
            for col in 1..4 {
                let v = board.get_value(4 * row + col);
                if left_value == 0 {
                    left_value = v;
                    continue;
                }
                if v == 0 {
                    continue;
                }
                if v < left_value {
                    left_right_inversions += 1;
                } else if v > left_value {
                    right_left_inversions += 1;
                }
                left_value = v;
            }
            inversions += min(left_right_inversions, right_left_inversions);
        }
        inversions
    }

    fn column_inversions(&self, board: Board) -> usize {
        let mut inversions = 0;
        for col in 0..4 {
            let mut up_value = board.get_value(col);
            let mut up_bottom_inversions = 0;
            let mut bottom_up_inversions = 0;
            for row in 1..4 {
                let v = board.get_value(4 * row + col);
                if up_value == 0 {
                    up_value = v;
                    continue;
                }
                if v == 0 {
                    continue;
                }
                if v < up_value {
                    up_bottom_inversions += 1;
                } else if v > up_value {
                    bottom_up_inversions += 1;
                }
                up_value = v;
            }
            inversions += min(up_bottom_inversions, bottom_up_inversions);
        }
        inversions
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
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);
        let evaluator = InversionEvaluator {};

        // When
        let row_inversions = evaluator.row_inversions(board);
        let col_inversions = evaluator.column_inversions(board);
        assert_eq!(2, row_inversions);
        assert_eq!(3, col_inversions);
        let evaluation = evaluator.evaluate(board);
        assert_eq!(1. - 5. / 16., evaluation);
    }
}
