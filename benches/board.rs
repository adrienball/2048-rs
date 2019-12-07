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

fn get_row(c: &mut Criterion) {
    #[rustfmt::skip]
    let board = Board::from(vec![
        8, 8, 0, 8,
        8, 0, 8, 8,
        0, 8, 8, 0,
        8, 8, 0, 0,
    ]);
    c.bench_function("Get row", move |b| b.iter(|| board.get_row(2)));
}

fn get_column(c: &mut Criterion) {
    #[rustfmt::skip]
    let board = Board::from(vec![
        8, 8, 0, 8,
        8, 0, 8, 8,
        0, 8, 8, 0,
        8, 8, 0, 0,
    ]);
    c.bench_function("Get column", move |b| b.iter(|| board.get_column(2)));
}

criterion_group!(benches, move_left, move_right, move_up, move_down, get_row, get_column);
criterion_main!(benches);
