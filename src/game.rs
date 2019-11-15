use crate::board::{Board, Direction};
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct Game {
    pub board: Board,
    pub proba_4: f32,
    rng: ThreadRng,
}

impl Game {
    pub fn play(&mut self, direction: Direction) {
        self.board = self.board.move_to(direction);
    }

    pub fn populate_new_tile(&mut self) {
        let rnd_value: f32 = self.rng.gen();
        let populated_value = if rnd_value < self.proba_4 { 4 } else { 2 };
        let empty_tiles = self.board.empty_tiles_indices();
        let mut rnd_idx: usize = self.rng.gen();
        rnd_idx = rnd_idx % empty_tiles.len();
        self.board = self.board.set_value(empty_tiles[rnd_idx], populated_value);
    }
}

pub struct GameBuilder {
    initial_board: Option<Board>,
    proba_4: f32,
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            initial_board: None,
            proba_4: 0.2,
        }
    }
}

impl GameBuilder {
    pub fn initial_board(mut self, board: impl Into<Option<Board>>) -> Self {
        self.initial_board = board.into();
        self
    }

    pub fn proba_4(mut self, proba: f32) -> Self {
        self.proba_4 = proba;
        self
    }

    pub fn build(self) -> Game {
        let proba_4 = self.proba_4;
        let mut rng = rand::thread_rng();
        let board = self.initial_board.unwrap_or_else(|| {
            let rand_value: f32 = rng.gen();
            let initial_value = if rand_value < proba_4 { 4 } else { 2 };
            let rand_idx: u8 = rng.gen();
            let board = Board::default();
            board.set_value(rand_idx % 16, initial_value)
        });
        Game {
            board,
            proba_4: self.proba_4,
            rng,
        }
    }
}
