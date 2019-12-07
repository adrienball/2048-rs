#[macro_use]
extern crate criterion;
extern crate play_2048;

use criterion::Criterion;
use play_2048::board::Board;
use play_2048::evaluators::*;
use play_2048::solver::SolverBuilder;

fn next_best_move(c: &mut Criterion) {
    let mut solver = SolverBuilder::default()
        .board_evaluator(PrecomputedBoardEvaluator::new(
            CombinedBoardEvaluator::default()
                .add(MonotonicityEvaluator {
                    gameover_penalty: -300.,
                    monotonicity_power: 2,
                })
                .add(EmptyTileEvaluator {
                    gameover_penalty: 0.,
                    power: 2,
                })
                .add(AlignmentEvaluator {
                    gameover_penalty: 0.,
                    power: 2,
                }),
        ))
        .proba_4(0.1)
        .base_max_search_depth(3)
        .distinct_tiles_threshold(5)
        .min_branch_proba(0.0001)
        .build();

    #[rustfmt::skip]
    let board = Board::from(vec![
        128, 256, 512, 1024,
        64, 16, 8, 4,
        16, 4, 8, 4,
        4, 4, 8, 4,
    ]);
    c.bench_function("Compute next best move", move |b| {
        b.iter(|| solver.next_best_move(board))
    });
}

criterion_group!(benches, next_best_move,);
criterion_main!(benches);
