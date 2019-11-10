mod board;
pub mod error;
pub mod evaluators;
pub mod game;
pub mod strategy;
mod utils;

pub type GameResult<T> = Result<T, error::Error>;

fn main() {
    println!("Hello, world!");
}
