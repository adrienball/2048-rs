use crate::board::{Board, Direction};
use crate::evaluators::BoardEvaluator;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cmp::min;

pub struct Strategy {
    board_evaluator: Box<dyn BoardEvaluator>,
    max_exploration_depth: usize,
    rng: ThreadRng,
    max_draws: usize,
}

impl Strategy {
    pub fn new(
        evaluator: Box<dyn BoardEvaluator>,
        exploration_depth: usize,
        max_draws: usize,
    ) -> Self {
        Self {
            board_evaluator: evaluator,
            max_exploration_depth: exploration_depth,
            rng: thread_rng(),
            max_draws,
        }
    }

    pub fn next_best_move(&mut self, board: Board, proba_4: f32) -> Option<Direction> {
        Direction::all()
            .iter()
            .filter_map(|direction| {
                let new_board = board.move_to(*direction);
                if board == new_board {
                    return None;
                }
                let score = self.evaluate(new_board, 0, proba_4);
                Some((direction, score))
            })
            .max_by(|(_, lhs_score), (_, rhs_score)| lhs_score.partial_cmp(rhs_score).unwrap())
            .map(|(d, _)| *d)
    }

    fn evaluate(&mut self, board: Board, depth: usize, proba_4: f32) -> f32 {
        if depth == self.max_exploration_depth {
            return self.board_evaluator.evaluate(board);
        }
        let mut empty_tiles_indices = board.empty_tiles_indices();
        if empty_tiles_indices.is_empty() {
            return self.board_evaluator.evaluate(board);
        }
        empty_tiles_indices.shuffle(&mut self.rng);
        let scores_sum: f32 = empty_tiles_indices
            .iter()
            .take(self.max_draws)
            .map(|idx| {
                let rand_value: f32 = self.rng.gen();
                let drawn_value = if rand_value < proba_4 { 4 } else { 2 };
                let board_with_draw = board.set_value(*idx, drawn_value);
                Direction::all()
                    .iter()
                    .filter_map(|d| {
                        let new_board = board_with_draw.move_to(*d);
                        if board_with_draw == new_board {
                            return None;
                        }
                        Some(self.evaluate(new_board, depth + 1, proba_4))
                    })
                    .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
                    .unwrap_or(0.0)
            })
            .sum();
        scores_sum / min(self.max_draws, empty_tiles_indices.len()) as f32
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

        let mut strategy_shallow = Strategy::new(Box::new(DummyEvaluator {}), 0, 4);
        let mut strategy_deep = Strategy::new(Box::new(DummyEvaluator {}), 2, 4);

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
        assert_eq!(Some(Direction::Down), direction_shallow);
        assert_eq!(Some(Direction::Right), direction_deep);
    }
}
