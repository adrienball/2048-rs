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
                .combine(
                    MonotonicityEvaluator {
                        gameover_penalty: -300.,
                        monotonicity_power: 2,
                    },
                    1.0,
                )
                .combine(
                    EmptyTileEvaluator {
                        gameover_penalty: 0.,
                        power: 2,
                    },
                    200.0,
                )
                .combine(
                    AlignmentEvaluator {
                        gameover_penalty: 0.,
                        power: 2,
                    },
                    500.0,
                ),
        ))
        .proba_4(0.1)
        .base_max_search_depth(4)
        .min_branch_proba(0.0001)
        .build();

    #[rustfmt::skip]
    let board = Board::from(vec![
        128, 256, 512, 2048,
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
