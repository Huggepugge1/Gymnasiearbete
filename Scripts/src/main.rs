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
    pub start_square: i8,
    pub end_square: i8,
    pub needs_refresh: bool,
    pub frame: i8,

    pub board: Box<board::Board>,
    pub difficulty: i8
}

impl MainState {
    pub fn new() -> Self {
        MainState {
            selected_squares: [false; 64],
            number_of_selected_squares: 0,
            start_square: -1,
            end_square: -1,
            needs_refresh: true,
            frame: 0,

            board: board::create_board(),
            difficulty: ai::EASY
        }
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("yes", "Yes");
    let (mut ctx, event_loop) = cb.build()?;

    graphics::set_window_title(&ctx, "hello");
    graphics::set_drawable_size(&mut ctx, 800.0, 800.0)?;

    let state: MainState = MainState::new();
    event::run(ctx, event_loop, state);
    Ok(())
}
