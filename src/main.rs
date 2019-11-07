mod board;
mod error;

pub type GameResult<T> = Result<T, error::Error>;

fn main() {
    println!("Hello, world!");
}
