use crate::board::Board;
use std::cmp::{min, Ordering};

/// Evaluate a `Board` by mapping it to a number. The higher the number, the better the board
/// state.
pub trait BoardEvaluator {
    fn evaluate(&self, board: Board) -> f32;
    fn gameover_penalty(&self) -> f32;
}

/// Evaluate a `Board` by evaluating independently each row and column and summing the results
pub trait RowColumnEvaluator {
    fn evaluate_row(&self, row: u16) -> f32;
    fn gameover_penalty(&self) -> f32;

    fn get_statistics(&self) -> EvaluatorStats {
        let values: Vec<f32> = (0..(std::u16::MAX as usize + 1))
            .map(|row| self.evaluate_row(row as u16))
            .collect();
        let mut max_value = f32::MIN;
        let mut min_value = f32::MAX;
        for v in values.iter() {
            if *v > max_value {
                max_value = *v;
            }
            if *v < min_value {
                min_value = *v;
            }
        }
        let mean = values.iter().sum::<f32>() / (std::u16::MAX as f32 + 1.);
        EvaluatorStats {
            max_value,
            min_value,
            mean,
            standard_dev: 0.,
        }
    }
}

impl<T> BoardEvaluator for T
where
    T: RowColumnEvaluator,
{
    fn evaluate(&self, board: Board) -> f32 {
        (0..4)
            .map(|i| self.evaluate_row(board.get_row(i)) + self.evaluate_row(board.get_column(i)))
            .sum()
    }

    fn gameover_penalty(&self) -> f32 {
        self.gameover_penalty()
    }
}

#[derive(Debug)]
pub struct EvaluatorStats {
    pub max_value: f32,
    pub min_value: f32,
    pub mean: f32,
    pub standard_dev: f32,
}

/// `BoardEvaluator` implementation which encapsulates a `RowColumnEvaluator` and pre-computes
/// values for any possible row / column. These values are stored in a simple vector which is
/// indexed by the `u16` representation of each row.
pub struct PrecomputedBoardEvaluator {
    row_cache: Vec<f32>,
    gameover_penalty: f32,
}

impl PrecomputedBoardEvaluator {
    pub fn new<T>(evaluator: T) -> Self
    where
        T: RowColumnEvaluator,
    {
        let row_cache = (0..(std::u16::MAX as usize + 1))
            .map(|row| evaluator.evaluate_row(row as u16))
            .collect();
        Self {
            row_cache,
            gameover_penalty: evaluator.gameover_penalty(),
        }
    }
}

/// `BoardEvaluator` implementation which combines multiple board evaluators by summing
/// their evaluations
#[derive(Default)]
pub struct CombinedBoardEvaluator {
    /// evaluators along with their weight
    evaluators: Vec<(Box<dyn RowColumnEvaluator>, f32)>,
}

impl CombinedBoardEvaluator {
    pub fn combine<T>(mut self, evaluator: T, weight: f32) -> Self
    where
        T: RowColumnEvaluator + 'static,
    {
        self.evaluators.push((Box::new(evaluator), weight));
        self
    }
}

impl RowColumnEvaluator for CombinedBoardEvaluator {
    fn evaluate_row(&self, row: u16) -> f32 {
        self.evaluators
            .iter()
            .map(|(evaluator, weight)| weight * evaluator.evaluate_row(row))
            .sum()
    }

    fn gameover_penalty(&self) -> f32 {
        self.evaluators
            .iter()
            .map(|(evaluator, _)| evaluator.gameover_penalty())
            .sum()
    }
}

impl BoardEvaluator for PrecomputedBoardEvaluator {
    fn evaluate(&self, board: Board) -> f32 {
        (0..4)
            .map(|i| {
                self.row_cache[board.get_row(i) as usize]
                    + self.row_cache[board.get_column(i) as usize]
            })
            .sum()
    }

    fn gameover_penalty(&self) -> f32 {
        self.gameover_penalty
    }
}

/// A simple implementation of `BoardEvaluator` which evaluates a board by simply computing
/// the number of empty tiles.
pub struct EmptyTileEvaluator {
    pub gameover_penalty: f32,
    pub power: u32,
}

impl Default for EmptyTileEvaluator {
    fn default() -> Self {
        Self {
            gameover_penalty: 0.0,
            power: 1,
        }
    }
}

impl RowColumnEvaluator for EmptyTileEvaluator {
    fn evaluate_row(&self, row: u16) -> f32 {
        let mut nb_empty: u32 = 0;
        let mut row = row;
        for _ in 0..4 {
            if row.trailing_zeros() >= 4 {
                nb_empty += 1;
            }
            row >>= 4;
        }
        nb_empty.pow(self.power) as f32
    }

    fn gameover_penalty(&self) -> f32 {
        self.gameover_penalty
    }
}

/// `BoardEvaluator` implementation which computes the number of tiles alignments
pub struct AlignmentEvaluator {
    pub gameover_penalty: f32,
    pub power: u32,
}

impl Default for AlignmentEvaluator {
    fn default() -> Self {
        Self {
            gameover_penalty: 0.,
            power: 2,
        }
    }
}

impl RowColumnEvaluator for AlignmentEvaluator {
    fn evaluate_row(&self, row: u16) -> f32 {
        let mut row = row;
        let mut right_value = row & 0b1111;
        row >>= 4;
        let mut nb_aligned: u32 = 0;
        for _ in 1..4 {
            let value = row & 0b1111;
            if value == right_value && value != 0 {
                nb_aligned += 1;
            }
            right_value = value;
            row >>= 4;
        }
        nb_aligned.pow(self.power) as f32
    }

    fn gameover_penalty(&self) -> f32 {
        self.gameover_penalty
    }
}

/// `BoardEvaluator` implementation which computes inversions on rows and columns, add them, and
/// then takes the negative value of it.
/// Inversions are computed by summing the differences between exponents which appear in the
/// wrong order in each row / column
pub struct MonotonicityEvaluator {
    pub gameover_penalty: f32,
    pub monotonicity_power: u32,
}

impl Default for MonotonicityEvaluator {
    fn default() -> Self {
        Self {
            gameover_penalty: -300.,
            monotonicity_power: 2,
        }
    }
}

impl RowColumnEvaluator for MonotonicityEvaluator {
    fn evaluate_row(&self, row: u16) -> f32 {
        let mut left_value: u32 = (row >> 12) as u32;
        let mut left_right_inversions = 0;
        let mut right_left_inversions = 0;
        for col in 1..4 {
            let v: u32 = ((row >> (4 * (3 - col))) & 0b1111) as u32;
            match v.cmp(&left_value) {
                Ordering::Less => {
                    left_right_inversions +=
                        left_value.pow(self.monotonicity_power) - v.pow(self.monotonicity_power);
                }
                Ordering::Greater => {
                    right_left_inversions +=
                        v.pow(self.monotonicity_power) - left_value.pow(self.monotonicity_power);
                }
                Ordering::Equal => {}
            }
            left_value = v;
        }
        -(min(left_right_inversions, right_left_inversions) as f32)
    }

    fn gameover_penalty(&self) -> f32 {
        self.gameover_penalty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let evaluator = CombinedBoardEvaluator::default()
            .combine(
                MonotonicityEvaluator {
                    gameover_penalty: -200000.,
                    monotonicity_power: 4,
                },
                1.0,
            )
            .combine(
                EmptyTileEvaluator {
                    gameover_penalty: 0.,
                    power: 1,
                },
                200.0,
            )
            .combine(
                AlignmentEvaluator {
                    gameover_penalty: 0.,
                    power: 1,
                },
                500.0,
            );
        eprintln!("evaluator.stats = {:?}", evaluator.get_statistics());
    }

    #[test]
    fn test_empty_tile_evaluator() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            0, 2, 0, 0,
            0, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);
        let evaluator = EmptyTileEvaluator {
            gameover_penalty: 0.,
            power: 2,
        };

        // When
        assert_eq!(4., evaluator.evaluate_row(board.get_row(1)));
    }

    #[test]
    fn test_alignment_evaluator() {
        // Given
        #[rustfmt::skip]
            let vec_board = vec![
            0, 2, 0, 0,
            16, 16, 256, 256,
            0, 8, 0, 8,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);
        let evaluator = AlignmentEvaluator {
            gameover_penalty: 0.,
            power: 2,
        };

        // When / Then
        assert_eq!(0., evaluator.evaluate_row(board.get_row(0)));
        assert_eq!(4., evaluator.evaluate_row(board.get_row(1)));
        assert_eq!(0., evaluator.evaluate_row(board.get_row(2)));
    }

    #[test]
    fn test_monotonicity_evaluator() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            2, 4, 2, 4,
            8, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64,
        ];
        let board = Board::from(vec_board);
        let evaluator = MonotonicityEvaluator {
            gameover_penalty: -300.,
            monotonicity_power: 2,
        };

        // When
        let row_inversions_1 = evaluator.evaluate_row(board.get_row(1));
        let row_inversions_2 = evaluator.evaluate_row(board.get_row(2));
        let col_inversions = evaluator.evaluate_row(board.get_column(1));

        // Then
        assert_eq!(-64., row_inversions_1);
        assert_eq!(-96., row_inversions_2);
        assert_eq!(-61., col_inversions);
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
        let evaluator = PrecomputedBoardEvaluator::new(MonotonicityEvaluator::default());

        // When
        let evaluation = evaluator.evaluate(board);

        // Then
        assert_eq!(MonotonicityEvaluator::default().evaluate(board), evaluation);
    }

    #[test]
    fn test_combined_evaluator() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            2, 4, 2, 4,
            8, 0, 0, 512,
            1024, 2, 16, 0,
            8, 2, 16, 64,
        ];
        let board = Board::from(vec_board);
        let evaluator = CombinedBoardEvaluator::default()
            .combine(
                EmptyTileEvaluator {
                    gameover_penalty: 0.,
                    power: 2,
                },
                2.0,
            )
            .combine(
                MonotonicityEvaluator {
                    gameover_penalty: 0.,
                    monotonicity_power: 2,
                },
                1.0,
            );

        // When
        let evaluation_1 = evaluator.evaluate_row(board.get_row(1));
        let evaluation_2 = evaluator.evaluate_row(board.get_row(2));

        // Then
        assert_eq!(-9. + 2. * 4., evaluation_1);
        assert_eq!(-15. + 2. * 1., evaluation_2);
    }
}
