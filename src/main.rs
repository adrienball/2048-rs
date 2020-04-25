use crate::board::{Board, Direction};
use crate::evaluators::*;
use crate::game::{Game, GameBuilder};
use crate::solver::{Solver, SolverBuilder};
use clap::{App, AppSettings, Arg, ArgMatches};
use std::io::{stdout, StdoutLock, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, cursor, style};

mod board;
mod evaluators;
mod game;
mod solver;
mod utils;

mod graphics {
    pub const CONTROLS: &str = "╓─────────┬─────CONTROLS─────────╖\n\r\
                                ║ ← ↑ → ↓ | move tiles           ║\n\r\
                                ║      p  | use AI for next move ║\n\r\
                                ║      a  | toggle AI autoplay   ║\n\r\
                                ║      q  | quit                 ║\n\r\
                                ╚═════════╧══════════════════════╝";
}

fn init_logger() {
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init()
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("2048")
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
}

fn get_solver(matches: &ArgMatches) -> Solver {
    let penalty = f32::from_str(matches.value_of("gameover_penalty").unwrap()).unwrap();
    let proba_4 = f32::from_str(matches.value_of("proba_4").unwrap()).unwrap();
    SolverBuilder::default()
        .board_evaluator(PrecomputedBoardEvaluator::new(
            CombinedBoardEvaluator::default()
                .combine(MonotonicityEvaluator {
                    gameover_penalty: penalty,
                    monotonicity_power: 2,
                })
                .combine(EmptyTileEvaluator {
                    gameover_penalty: 0.,
                    power: 2,
                })
                .combine(AlignmentEvaluator {
                    gameover_penalty: 0.,
                    power: 2,
                }),
        ))
        .proba_4(proba_4)
        .base_max_search_depth(usize::from_str(matches.value_of("depth").unwrap()).unwrap())
        .distinct_tiles_threshold(
            usize::from_str(matches.value_of("distinct_tiles_threshold").unwrap()).unwrap(),
        )
        .min_branch_proba(f32::from_str(matches.value_of("min_branch_proba").unwrap()).unwrap())
        .build()
}

fn update_board(board: Board, stdout: &mut StdoutLock) {
    write!(
        stdout,
        "{}{}\n{}{}",
        cursor::Goto(1, 5),
        board,
        graphics::CONTROLS,
        cursor::Hide
    )
    .unwrap();
}

fn play(game: &mut Game, direction: Direction, stdout: &mut StdoutLock) {
    let previous_board = game.board;
    game.play(direction);
    if previous_board == game.board {
        return;
    }
    update_board(game.board, stdout);
    game.populate_new_tile();
    update_board(game.board, stdout);
}

fn main() {
    init_logger();
    let matches = get_app().get_matches();
    let mut solver = get_solver(&matches);
    let proba_4 = f32::from_str(matches.value_of("proba_4").unwrap()).unwrap();

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    #[rustfmt::skip]
    let board: Board = Board::from(vec![
        0, 2, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ]);

    let mut game = GameBuilder::default()
        .initial_board(board)
        .proba_4(proba_4)
        .build();

    update_board(game.board, &mut stdout);
    game.populate_new_tile();
    update_board(game.board, &mut stdout);
    let mut autoplay = false;

    let mut before = Instant::now();
    loop {
        let interval = 10;
        let now = Instant::now();
        let dt = now.duration_since(before).subsec_millis() as u64;

        if dt < interval {
            sleep(Duration::from_millis(interval - dt));
            continue;
        }
        before = now;

        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                Key::Char('q') => break,
                Key::Ctrl('c') => break,
                Key::Left => play(&mut game, Direction::Left, &mut stdout),
                Key::Right => play(&mut game, Direction::Right, &mut stdout),
                Key::Up => play(&mut game, Direction::Up, &mut stdout),
                Key::Down => play(&mut game, Direction::Down, &mut stdout),
                Key::Char('p') => {
                    if let Some(next_move) = solver.next_best_move(game.board) {
                        play(&mut game, next_move, &mut stdout)
                    }
                }
                Key::Char('a') => autoplay = !autoplay,
                _ => continue,
            };
        } else if autoplay {
            if let Some(next_move) = solver.next_best_move(game.board) {
                play(&mut game, next_move, &mut stdout)
            }
        }
    }

    write!(
        stdout,
        "{}{}{}{}",
        clear::All,
        style::Reset,
        cursor::Goto(1, 1),
        cursor::Show,
    )
    .unwrap();
}
