use std::thread;

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
    pub needs_refresh: bool,
    pub frame: u64,

    pub board: board::Board
}

impl MainState {
    pub fn new() -> Self {
        MainState {
            selected_squares: [false; 64],
            number_of_selected_squares: 0,
            start_square: -1,
            end_square: -1,
            needs_refresh: true,
            board: board::create_board(),
            frame: 0
        }
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("yes", "Yes");
    let (mut ctx, event_loop) = cb.build()?;

    graphics::set_window_title(&ctx, "hello");
    graphics::set_drawable_size(&mut ctx, 800.0, 800.0)?;

    let mut state: MainState = MainState::new();
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });
    event::run(ctx, event_loop, state);
    Ok(())
}
