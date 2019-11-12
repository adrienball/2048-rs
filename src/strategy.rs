use crate::board::{Board, Direction};
use crate::evaluators::BoardEvaluator;

pub struct Strategy {
    board_evaluator: Box<dyn BoardEvaluator>,
    proba_4: f32,
    max_search_depth: usize,
    gameover_penalty: f32,
    min_branch_proba: f32,
}

impl Strategy {
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
        }
    }

    pub fn next_best_move(&mut self, board: Board) -> Option<Direction> {
        Direction::all()
            .iter()
            .filter_map(|direction| {
                let new_board = board.move_to(*direction);
                if board == new_board {
                    return None;
                }
                let score = self.evaluate(new_board, self.max_search_depth as usize);
                Some((direction, score))
            })
            .max_by(|(_, lhs_score), (_, rhs_score)| lhs_score.partial_cmp(rhs_score).unwrap())
            .map(|(d, _)| *d)
    }

    fn evaluate(&mut self, board: Board, depth: usize) -> f32 {
        if depth == 0 {
            let eval = self.board_evaluator.evaluate(board);
            return eval;
        }
        let empty_tiles_indices = board.empty_tiles_indices();
        let nb_empty_tiles = empty_tiles_indices.len();
        let proba_4 = self.proba_4;
        let scores_sum: f32 = empty_tiles_indices
            .into_iter()
            .flat_map(|idx| vec![(idx, 1, 1. - proba_4), (idx, 2, proba_4)].into_iter())
            .map(|(idx, draw, proba)| {
                let board_with_draw = board.set_value_by_exponent(idx, draw);
                let draw_score = Direction::all()
                    .iter()
                    .filter_map(|d| {
                        let new_board = board_with_draw.move_to(*d);
                        if board_with_draw == new_board {
                            return None;
                        }
                        Some(self.evaluate(new_board, depth - 1))
                    })
                    .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
                    .unwrap_or(self.gameover_penalty);
                draw_score * proba
            })
            .sum();
        scores_sum / nb_empty_tiles as f32
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

        let mut strategy = Strategy::new(Box::new(DummyEvaluator {}), 0., 2, 0., 0.);

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
