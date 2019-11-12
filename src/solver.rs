use crate::board::{Board, Direction};
use crate::evaluators::{BoardEvaluator, InversionEvaluator, PrecomputedEvaluator};
use fnv::FnvHashMap;

pub struct Solver {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_4: f32,
    max_search_depth: usize,
    gameover_penalty: f32,
    min_branch_proba: f32,
    transposition_table: FnvHashMap<Board, f32>,
}

pub struct SolverBuilder {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_4: f32,
    max_search_depth: usize,
    gameover_penalty: f32,
    min_branch_proba: f32,
}

impl Default for SolverBuilder {
    fn default() -> Self {
        Self {
            board_evaluator: Box::new(PrecomputedEvaluator::new(InversionEvaluator {})),
            proba_4: 0.1,
            max_search_depth: 5,
            gameover_penalty: -200.,
            min_branch_proba: 0.1 * 0.1,
        }
    }
}

impl SolverBuilder {
    pub fn board_evaluator<T>(mut self, evaluator: T) -> Self
    where
        T: BoardEvaluator + 'static,
    {
        self.board_evaluator = Box::new(evaluator);
        self
    }

    pub fn proba_4(mut self, proba_4: f32) -> Self {
        self.proba_4 = proba_4;
        self
    }

    pub fn max_search_depth(mut self, max_search_depth: usize) -> Self {
        self.max_search_depth = max_search_depth;
        self
    }

    pub fn gameover_penalty(mut self, penalty: f32) -> Self {
        self.gameover_penalty = penalty;
        self
    }

    pub fn min_branch_proba(mut self, proba: f32) -> Self {
        self.min_branch_proba = proba;
        self
    }

    pub fn build(self) -> Solver {
        Solver {
            board_evaluator: self.board_evaluator,
            proba_4: self.proba_4,
            max_search_depth: self.max_search_depth,
            gameover_penalty: self.gameover_penalty,
            min_branch_proba: self.min_branch_proba,
            transposition_table: Default::default(),
        }
    }
}

impl Solver {
    pub fn new(
        evaluator: Box<dyn BoardEvaluator>,
        proba_4: f32,
        max_search_depth: usize,
        gameover_penalty: f32,
        min_branch_proba: f32,
    ) -> Self {
        Self {
            board_evaluator: evaluator,
            proba_4,
            max_search_depth,
            gameover_penalty,
            min_branch_proba,
            transposition_table: FnvHashMap::default(),
        }
    }

    pub fn next_best_move(&mut self, board: Board) -> Option<Direction> {
        let empty_tiles = board.empty_tiles_indices().len();
        let max_depth = (1. - empty_tiles as f32 / 15.) * self.max_search_depth as f32;
        self.transposition_table.clear();
        self.eval_max(board, max_depth as usize, 1.0)
            .map(|(d, _)| d)
    }

    fn eval_max(
        &mut self,
        board: Board,
        depth: usize,
        branch_proba: f32,
    ) -> Option<(Direction, f32)> {
        Direction::all()
            .iter()
            .filter_map(|d| {
                let new_board = board.move_to(*d);
                if board == new_board {
                    return None;
                }
                Some((*d, self.eval_average(new_board, depth, branch_proba)))
            })
            .max_by(|(_, lhs), (_, rhs)| lhs.partial_cmp(rhs).unwrap())
    }

    fn eval_average(&mut self, board: Board, depth: usize, branch_proba: f32) -> f32 {
        if let Some(cached_value) = self.transposition_table.get(&board) {
            return *cached_value;
        }
        let average = if depth == 0 || branch_proba < self.min_branch_proba {
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
                        .eval_max(board_with_draw, depth - 1, branch_proba * proba)
                        .map(|(_, score)| score)
                        .unwrap_or(self.gameover_penalty);
                    max_score * proba
                })
                .sum();
            scores_sum / nb_empty_tiles as f32
        };
        self.transposition_table.insert(board, average);
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

        let mut strategy = SolverBuilder::default()
            .board_evaluator(DummyEvaluator {})
            .max_search_depth(2)
            .build();

        #[rustfmt::skip]
        let board: Board = Board::from(vec![
            4, 4, 0, 4,
            16, 0, 0, 2,
            0, 8, 0, 16,
            0, 8, 0, 16,
        ]);

        // When
        let direction = strategy.next_best_move(board);

        // Then
        assert_eq!(Some(Direction::Down), direction);
    }
}
