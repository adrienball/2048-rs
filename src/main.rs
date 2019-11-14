use crate::board::Board;
use crate::evaluators::*;
use crate::game::GameBuilder;
use crate::solver::SolverBuilder;
use clap::{App, AppSettings, Arg};
use log::info;
use std::str::FromStr;
use std::time::{Duration, Instant};

mod board;
pub mod error;
pub mod evaluators;
pub mod game;
pub mod solver;
mod utils;

pub type GameResult<T> = Result<T, error::Error>;

fn main() {
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();

    let matches = App::new("2048")
        .about("The famous 2048 game")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(
            Arg::with_name("proba_4")
                .short("p")
                .long("--proba-4")
                .takes_value(true)
                .default_value("0.1")
                .help("probability of drawing a 4 tile"),
        )
        .arg(
            Arg::with_name("depth")
                .short("d")
                .long("--depth")
                .takes_value(true)
                .default_value("3")
                .help("max search depth which will be used in the expectiminimax algorithm"),
        )
        .arg(
            Arg::with_name("gameover_penalty")
                .short("g")
                .long("--gameover-penalty")
                .takes_value(true)
                .default_value("-300")
                .help("penalty to apply to 'dead-end' branches"),
        )
        .arg(
            Arg::with_name("min_branch_proba")
                .short("m")
                .long("--min-branch-proba")
                .takes_value(true)
                .default_value("0.001")
                .help("minimum probability for a branch to be explored"),
        )
        .arg(
            Arg::with_name("distinct_tiles_threshold")
                .short("t")
                .long("--distinct-tiles-threshold")
                .takes_value(true)
                .default_value("5")
                .help(
                    "threshold, in terms of number of distinct tiles, which is used to adjust \
                     the effective max search depth",
                ),
        )
        .get_matches();

    let mut solver = SolverBuilder::default()
        .board_evaluator(PrecomputedEvaluator::new(InversionEvaluator {}))
        .proba_4(f32::from_str(matches.value_of("proba_4").unwrap()).unwrap())
        .base_max_search_depth(usize::from_str(matches.value_of("depth").unwrap()).unwrap())
        .distinct_tiles_threshold(
            usize::from_str(matches.value_of("distinct_tiles_threshold").unwrap()).unwrap(),
        )
        .gameover_penalty(f32::from_str(matches.value_of("gameover_penalty").unwrap()).unwrap())
        .min_branch_proba(f32::from_str(matches.value_of("min_branch_proba").unwrap()).unwrap())
        .build();

    #[rustfmt::skip]
    let board: Board = Board::from(vec![
        0, 2, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ]);

    let mut game = GameBuilder::default()
        .initial_board(board)
        .proba_4(0.1)
        .build();

    game = game.populate_new_tile();
    let mut max_score = 0;
    let mut moves_time = Duration::default();
    let mut moves = 0;

    loop {
        let now = Instant::now();
        let next_move = solver.next_best_move(game.board);
        let elapsed = now.elapsed();
        moves_time += elapsed;
        moves += 1;
        match next_move {
            None => {
                info!("Final board: {}", game.board);
                break;
            }
            Some(best_move) => {
                game = game.play(best_move);
                let score = game.score();
                if score > max_score {
                    max_score = score;
                    let avg_move_time = moves_time.as_millis() as f32 / moves as f32;
                    info!("New max score --> {}", score);
                    info!("Average time per move: {}ms", avg_move_time);
                }
                game = game.populate_new_tile();
            }
        }
    }
}
