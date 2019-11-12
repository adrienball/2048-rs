mod board;
pub mod error;
pub mod evaluators;
pub mod game;
pub mod solver;
mod utils;

pub type GameResult<T> = Result<T, error::Error>;

fn main() {
    println!("Hello, world!");
}
