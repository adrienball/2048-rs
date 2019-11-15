use crate::board::{Board, Direction};
use crate::evaluators::{BoardEvaluator, InversionEvaluator, PrecomputedEvaluator};
use fnv::FnvHashMap;
use std::cmp::max;

pub struct Solver {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_4: f32,
    base_max_search_depth: usize,
    gameover_penalty: f32,
    min_branch_proba: f32,
    distinct_tiles_threshold: usize,
    transposition_table: FnvHashMap<Board, (f32, usize)>,
}

pub struct SolverBuilder {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_4: f32,
    base_max_search_depth: usize,
    gameover_penalty: f32,
    min_branch_proba: f32,
    distinct_tiles_threshold: usize,
}

impl Default for SolverBuilder {
    fn default() -> Self {
        Self {
            board_evaluator: Box::new(PrecomputedEvaluator::new(InversionEvaluator {})),
            proba_4: 0.1,
            base_max_search_depth: 3,
            gameover_penalty: -200.,
            min_branch_proba: 0.1 * 0.1,
            distinct_tiles_threshold: 4,
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

    /// Sets the penalty to apply to 'dead-end' branches
    pub fn gameover_penalty(mut self, penalty: f32) -> Self {
        self.gameover_penalty = penalty;
        self
    }

    /// Sets the minimum probability for a branch to be explored
    pub fn min_branch_proba(mut self, proba: f32) -> Self {
        self.min_branch_proba = proba;
        self
    }

    /// Sets the threshold, in terms of number of distinct tiles, which is used to adjust the
    /// effective max search depth
    pub fn distinct_tiles_threshold(mut self, threshold: usize) -> Self {
        self.distinct_tiles_threshold = threshold;
        self
    }

    pub fn build(self) -> Solver {
        Solver {
            board_evaluator: self.board_evaluator,
            proba_4: self.proba_4,
            base_max_search_depth: self.base_max_search_depth,
            gameover_penalty: self.gameover_penalty,
            min_branch_proba: self.min_branch_proba,
            distinct_tiles_threshold: self.distinct_tiles_threshold,
            transposition_table: Default::default(),
        }
    }
}

impl Solver {
    pub fn new(
        evaluator: Box<dyn BoardEvaluator>,
        proba_4: f32,
        base_max_search_depth: usize,
        gameover_penalty: f32,
        min_branch_proba: f32,
        distinct_tiles_threshold: usize,
    ) -> Self {
        Self {
            board_evaluator: evaluator,
            proba_4,
            base_max_search_depth,
            gameover_penalty,
            min_branch_proba,
            distinct_tiles_threshold,
            transposition_table: FnvHashMap::default(),
        }
    }

    pub fn next_best_move(&mut self, board: Board) -> Option<Direction> {
        let max_depth = max(
            self.base_max_search_depth as isize,
            board.count_distinct_tiles() as isize - self.distinct_tiles_threshold as isize,
        );
        self.transposition_table.clear();
        self.eval_max(board, max_depth as usize, 1.0)
            .map(|(d, _)| d)
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
        if let Some((cached_value, cached_remaining_depth)) = self.transposition_table.get(&board) {
            if *cached_remaining_depth >= remaining_depth {
                return *cached_value;
            }
        }
        let average = if remaining_depth == 0 || branch_proba < self.min_branch_proba {
            self.board_evaluator.evaluate(board)
        } else {
            let empty_tiles_indices = board.empty_tiles_indices();
            let nb_empty_tiles = empty_tiles_indices.len();
            let proba_4 = self.proba_4;
            let scores_sum: f32 = empty_tiles_indices
                .into_iter()
                .flat_map(|idx| vec![(idx, 1, 1. - proba_4), (idx, 2, proba_4)].into_iter())
                .map(|(idx, draw, proba)| {
                    let board_with_draw = board.set_value_by_exponent(idx, draw);
                    let max_score = self
                        .eval_max(board_with_draw, remaining_depth - 1, branch_proba * proba)
                        .map(|(_, score)| score)
                        .unwrap_or(self.gameover_penalty);
                    max_score * proba
                })
                .sum();
            scores_sum / nb_empty_tiles as f32
        };
        self.transposition_table
            .insert(board, (average, remaining_depth));
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