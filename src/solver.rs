use crate::board::{Board, Direction};
use crate::evaluators::{BoardEvaluator, MonotonicityEvaluator, PrecomputedBoardEvaluator};
use fnv::FnvHashMap;
use std::cmp::max;

pub struct Solver {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_2: f32,
    proba_4: f32,
    base_max_search_depth: usize,
    min_branch_proba: f32,
    transposition_table: FnvHashMap<Board, (f32, f32)>,
}

pub struct SolverBuilder {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_4: f32,
    base_max_search_depth: usize,
    min_branch_proba: f32,
}

impl Default for SolverBuilder {
    fn default() -> Self {
        Self {
            board_evaluator: Box::new(PrecomputedBoardEvaluator::new(
                MonotonicityEvaluator::default(),
            )),
            proba_4: 0.1,
            base_max_search_depth: 3,
            min_branch_proba: 0.1 * 0.1,
        }
    }
}

impl SolverBuilder {
    /// Sets the `BoardEvaluator` implementation to use in the solver
    pub fn board_evaluator<T>(mut self, evaluator: T) -> Self
    where
        T: BoardEvaluator + 'static,
    {
        self.board_evaluator = Box::new(evaluator);
        self
    }

    /// Sets the probability weight associated to the draw of a 4 tile
    pub fn proba_4(mut self, proba_4: f32) -> Self {
        self.proba_4 = proba_4;
        self
    }

    /// Sets the max search depth which will be used in the expectiminimax algorithm
    /// This value is adjusted at each move to take into account the difficulty of the board.
    /// It is thus the max depth which will be used in easy configurations. The effective
    /// max depth will be higher for more difficult ones.
    pub fn base_max_search_depth(mut self, max_search_depth: usize) -> Self {
        self.base_max_search_depth = max_search_depth;
        self
    }

    /// Sets the minimum probability for a branch to be explored
    pub fn min_branch_proba(mut self, proba: f32) -> Self {
        self.min_branch_proba = proba;
        self
    }

    pub fn build(self) -> Solver {
        Solver {
            board_evaluator: self.board_evaluator,
            proba_2: 1. - self.proba_4,
            proba_4: self.proba_4,
            base_max_search_depth: self.base_max_search_depth,
            min_branch_proba: self.min_branch_proba,
            transposition_table: Default::default(),
        }
    }
}

impl Solver {
    pub fn next_best_move(&mut self, board: Board) -> Option<Direction> {
        let max_depth = self.compute_max_depth(board);
        self.transposition_table = FnvHashMap::default();
        self.eval_max(board, max_depth as usize, 1.0)
            .map(|(d, _)| d)
    }

    fn compute_max_depth(&self, board: Board) -> usize {
        let adjustment_factor = match board.max_value() {
            2048 => 5,
            4096 => 3,
            8192 => 3,
            16384 => 2,
            32768 => 1,
            _ => 7,
        };
        max(
            self.base_max_search_depth as isize,
            board.count_distinct_tiles() as isize - adjustment_factor,
        ) as usize
    }

    fn eval_max(
        &mut self,
        board: Board,
        remaining_depth: usize,
        branch_proba: f32,
    ) -> Option<(Direction, f32)> {
        Direction::all()
            .iter()
            .filter_map(|d| {
                let new_board = board.move_to(*d);
                if board == new_board {
                    return None;
                }
                Some((
                    *d,
                    self.eval_average(new_board, remaining_depth, branch_proba),
                ))
            })
            .max_by(|(_, lhs), (_, rhs)| lhs.partial_cmp(rhs).unwrap())
    }

    fn eval_average(&mut self, board: Board, remaining_depth: usize, branch_proba: f32) -> f32 {
        if remaining_depth == 0 || branch_proba < self.min_branch_proba {
            return self.board_evaluator.evaluate(board);
        }

        if let Some((cached_value, cached_proba)) = self.transposition_table.get(&board) {
            if *cached_proba >= branch_proba {
                return *cached_value;
            }
        }

        let empty_tiles_indices = board.empty_tiles_indices();
        let nb_empty_tiles = board.count_empty_tiles() as f32;
        let proba_2 = self.proba_2;
        let proba_4 = self.proba_4;
        let scores_sum: f32 = empty_tiles_indices
            .map(|idx| {
                let board_with_2 = board.set_value_by_exponent(idx, 1);
                let board_with_4 = board.set_value_by_exponent(idx, 2);
                let max_score_2 = self
                    .eval_max(
                        board_with_2,
                        remaining_depth - 1,
                        branch_proba * proba_2 / nb_empty_tiles,
                    )
                    .map(|(_, score)| score)
                    .unwrap_or_else(|| self.board_evaluator.gameover_penalty());
                let max_score_4 = self
                    .eval_max(
                        board_with_4,
                        remaining_depth - 1,
                        branch_proba * proba_4 / nb_empty_tiles,
                    )
                    .map(|(_, score)| score)
                    .unwrap_or_else(|| self.board_evaluator.gameover_penalty());
                max_score_2 * proba_2 + max_score_4 * proba_4
            })
            .sum();
        let average = scores_sum / nb_empty_tiles as f32;
        self.transposition_table
            .insert(board, (average, branch_proba));
        average
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_best_move() {
        // Given
        struct DummyEvaluator;
        impl BoardEvaluator for DummyEvaluator {
            fn evaluate(&self, board: Board) -> f32 {
                board.max_value() as f32 / 32768.
            }

            fn gameover_penalty(&self) -> f32 {
                0.
            }
        }

        let mut solver = SolverBuilder::default()
            .board_evaluator(DummyEvaluator {})
            .base_max_search_depth(2)
            .build();

        #[rustfmt::skip]
        let board: Board = Board::from(vec![
            4, 4, 0, 4,
            16, 0, 0, 2,
            0, 8, 0, 16,
            0, 8, 0, 16,
        ]);

        // When
        let direction = solver.next_best_move(board);

        // Then
        assert_eq!(Some(Direction::Down), direction);
    }
}
