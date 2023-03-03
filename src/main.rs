use ggez;
use ggez::GameResult;
use ggez::graphics;
use ggez::event;

mod gui;
mod board;
mod moves;
mod ai;

pub struct MainState {
    pub selected_squares: [bool; 64],
    pub number_of_selected_squares: u8,
    pub current_move: moves::Move,
    pub needs_refresh: bool,
    pub frame: i8,

    pub board: board::Board,
    pub difficulty: i8,
    pub last_move: moves::Move,
}

impl MainState {
    pub fn new() -> Self {
        MainState {
            selected_squares: [false; 64],
            number_of_selected_squares: 0,
            current_move: moves::Move {
                start: 0,
                end: 0,
            },
            needs_refresh: true,
            frame: 0,

            board: board::create_board(),
            difficulty: ai::HARD,
            last_move: moves::Move {
                start: -1,
                end: -1,
            },
        }
    }
}

#[allow(unreachable_code)]
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("yes", "Yes");
    let (mut ctx, event_loop) = cb.build()?;

    graphics::set_window_title(&ctx, "hello");
    graphics::set_drawable_size(&mut ctx, 800.0, 800.0)?;

    let state: MainState = MainState::new();
    event::run(ctx, event_loop, state);
    Ok(())
}
