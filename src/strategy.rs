use crate::board::{Board, Direction};
use crate::evaluators::BoardEvaluator;

pub struct Strategy {
    board_evaluator: Box<dyn BoardEvaluator>,
    exploration_depth: usize,
}

impl Strategy {
    pub fn new(evaluator: Box<dyn BoardEvaluator>, exploration_depth: usize) -> Self {
        Self {
            board_evaluator: evaluator,
            exploration_depth,
        }
    }

    pub fn next_best_move(&self, board: Board, proba_4: f32) -> Direction {
        *Direction::all()
            .iter()
            .map(|direction| {
                let new_board = board.move_to(*direction);
                let score = self.evaluate(new_board, 0, proba_4);
                (direction, score)
            })
            .max_by(|(_, lhs_score), (_, rhs_score)| lhs_score.partial_cmp(rhs_score).unwrap())
            .unwrap()
            .0
    }

    fn evaluate(&self, board: Board, depth: usize, proba_4: f32) -> f32 {
        if depth == self.exploration_depth {
            return self.board_evaluator.evaluate(board);
        }
        let empty_tiles_indices = board.empty_tiles_indices();
        if empty_tiles_indices.len() == 0 {
            return self.board_evaluator.evaluate(board);
        }
        let draws = self.generate_all_possible_draws(board, &empty_tiles_indices, proba_4);
        let scores_sum: f32 = draws
            .map(|(board, proba)| {
                let max_value = Direction::all()
                    .iter()
                    .map(|d| self.evaluate(board.move_to(*d), depth + 1, proba_4))
                    .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
                    .unwrap();
                proba * max_value
            })
            .sum();
        scores_sum / empty_tiles_indices.len() as f32
    }

    fn generate_all_possible_draws<'a>(
        &self,
        board: Board,
        empty_tiles_indices: &'a Vec<u8>,
        proba_4: f32,
    ) -> impl Iterator<Item = (Board, f32)> + 'a {
        empty_tiles_indices.iter().flat_map(move |idx| {
            let drawn_2 = board.set_value(*idx, 2);
            let drawn_4 = board.set_value(*idx, 4);
            vec![(drawn_2, 1. - proba_4), (drawn_4, proba_4)].into_iter()
        })
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

        let strategy_shallow = Strategy::new(Box::new(DummyEvaluator {}), 0);
        let strategy_deep = Strategy::new(Box::new(DummyEvaluator {}), 2);

        #[rustfmt::skip]
        let board: Board = Board::from(vec![
            16, 2, 8, 8,
            16, 0, 0, 0,
            0, 8, 0, 8,
            0, 2, 0, 16,
        ]);

        // When
        let direction_shallow = strategy_shallow.next_best_move(board, 0.0);
        let direction_deep = strategy_deep.next_best_move(board, 0.0);

        // Then
        assert_eq!(Direction::Down, direction_shallow);
        assert_eq!(Direction::Right, direction_deep);
    }
}
