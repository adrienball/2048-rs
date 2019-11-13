use crate::board::Board;
use std::cmp::min;

/// Evaluate a `Board` by mapping it to a number. The higher the number, the better the board
/// state.
pub trait BoardEvaluator {
    fn evaluate(&self, board: Board) -> f32;
}

/// Evaluate a `Board` by evaluating independently each row and column and summing the results
pub trait RowColumnEvaluator {
    fn evaluate_row(&self, row: u16) -> f32;
}

impl<T> BoardEvaluator for T
where
    T: RowColumnEvaluator,
{
    fn evaluate(&self, board: Board) -> f32 {
        (0..4)
            .into_iter()
            .map(|i| self.evaluate_row(board.get_row(i)) + self.evaluate_row(board.get_column(i)))
            .sum()
    }
}

/// `BoardEvaluator` implementation which encapsulates a `RowColumnEvaluator` and pre-computes
/// values for any possible row / column. These values are stored in a simple vector which is
/// indexed by the `u16` representation of each row.
pub struct PrecomputedEvaluator {
    row_cache: Vec<f32>,
}

impl PrecomputedEvaluator {
    pub fn new<T>(evaluator: T) -> Self
    where
        T: RowColumnEvaluator,
    {
        let row_cache = (0..(std::u16::MAX as usize + 1))
            .into_iter()
            .map(|row| evaluator.evaluate_row(row as u16))
            .collect();
        Self { row_cache }
    }
}

impl BoardEvaluator for PrecomputedEvaluator {
    fn evaluate(&self, board: Board) -> f32 {
        (0..4)
            .into_iter()
            .map(|i| {
                self.row_cache[board.get_row(i) as usize]
                    + self.row_cache[board.get_column(i) as usize]
            })
            .sum()
    }
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

/// `BoardEvaluator` implementation which computes inversions on rows and columns, add them, and
/// then takes the negative value of it.
/// Inversions are computed by summing the differences between exponents which appear in the
/// wrong order in each row / column
pub struct InversionEvaluator;

impl RowColumnEvaluator for InversionEvaluator {
    fn evaluate_row(&self, row: u16) -> f32 {
        let mut left_value = row >> 12;
        let mut left_right_inversions = 0;
        let mut right_left_inversions = 0;
        for col in 1..4 {
            let v = (row >> 4 * (3 - col)) & 0b1111;
            if v < left_value {
                left_right_inversions += left_value - v;
            } else if v > left_value {
                right_left_inversions += v - left_value;
            }
            left_value = v;
        }
        -(min(left_right_inversions, right_left_inversions) as f32)
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
        let row_inversions = evaluator.evaluate_row(board.get_row(1));
        let col_inversions = evaluator.evaluate_row(board.get_column(1));
        let evaluation = evaluator.evaluate(board);
        assert_eq!(-8., row_inversions);
        assert_eq!(-7., col_inversions);
        assert_eq!(-19. - 24., evaluation);
    }

    #[test]
    fn test_precomputed_inversion_evaluator() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            2, 4, 2, 4,
            8, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64,
        ];
        let board = Board::from(vec_board);
        let evaluator = PrecomputedEvaluator::new(InversionEvaluator {});

        // When
        let evaluation = evaluator.evaluate(board);
        assert_eq!(-19. - 24., evaluation);
    }
}
