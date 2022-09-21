use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;

mod gui;
mod board;
mod moves;

const WHITE: u8 = 8;
const BLACK: u8 = 16;

pub struct MainState {
    pub selected_squares: [bool; 64],
    pub number_of_selected_squares: u8,
    pub start_square: i8,
    pub end_square: i8,
    pub needs_refresh: bool
}

impl MainState {
    pub fn new() -> Self {
        MainState {
            selected_squares: [false; 64],
            number_of_selected_squares: 0,
            start_square: -1,
            end_square: -1,
            needs_refresh: true
        }
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("yes", "Yes");
    let (mut ctx, event_loop) = cb.build()?;

    graphics::set_window_title(&ctx, "hello");
    graphics::set_drawable_size(&mut ctx, 800.0, 800.0)?;

    let mut state = MainState::new();
    event::run(ctx, event_loop, state);
    Ok(())
}
