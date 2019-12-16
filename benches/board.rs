#[macro_use]
extern crate criterion;
extern crate play_2048;

use criterion::Criterion;
use play_2048::board::{Board, Direction};

fn move_left(c: &mut Criterion) {
    #[rustfmt::skip]
    let board = Board::from(vec![
        8, 8, 0, 8,
        8, 0, 8, 8,
        0, 8, 8, 0,
        8, 8, 0, 0,
    ]);
    c.bench_function("Move left", move |b| {
        b.iter(|| board.move_to(Direction::Left))
    });
}

fn move_right(c: &mut Criterion) {
    #[rustfmt::skip]
    let board = Board::from(vec![
        8, 8, 0, 8,
        8, 0, 8, 8,
        0, 8, 8, 0,
        8, 8, 0, 0,
    ]);
    c.bench_function("Move right", move |b| {
        b.iter(|| board.move_to(Direction::Right))
    });
}

fn move_up(c: &mut Criterion) {
    #[rustfmt::skip]
    let board = Board::from(vec![
        8, 8, 0, 8,
        8, 0, 8, 8,
        0, 8, 8, 0,
        8, 8, 0, 0,
    ]);
    c.bench_function("Move up", move |b| b.iter(|| board.move_to(Direction::Up)));
}

fn move_down(c: &mut Criterion) {
    #[rustfmt::skip]
    let board = Board::from(vec![
        8, 8, 0, 8,
        8, 0, 8, 8,
        0, 8, 8, 0,
        8, 8, 0, 0,
    ]);
    c.bench_function("Move down", move |b| {
        b.iter(|| board.move_to(Direction::Down))
    });
}

criterion_group!(benches, move_left, move_right, move_up, move_down);
criterion_main!(benches);
