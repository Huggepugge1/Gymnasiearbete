use ggez;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::input::mouse::MouseButton;
use ggez::graphics;
use ggez::mint;

const square_size: f32 = 100.0;
const square_y: f32 = 0.75 * square_size;

use crate::MainState;

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32) {
        self.needs_refresh = true;
        let square: i8 = (((_x / square_size) as u32 % 8) + ((((800.0 - _y) / square_size) as u32 % 8) * 8)) as i8;
        if self.selected_squares[square as usize] == false && self.number_of_selected_squares < 2 {
            if self.number_of_selected_squares == 0 {
                self.start_square = square;
            } else {
                self.end_square = square;
            }
            
            self.selected_squares[square as usize] = true;
            self.number_of_selected_squares += 1;

        } else if self.selected_squares[square as usize] == true {
            self.selected_squares[square as usize] = false;
            self.number_of_selected_squares -= 1;

            if self.number_of_selected_squares == 0 {
                self.start_square = -1;
            } else {
                if self.start_square == square {
                    self.start_square = self.end_square;
                    self.end_square = -1;
                } else {
                    self.end_square = -1;
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if !self.needs_refresh {
            return Ok(());
        }
        graphics::clear(ctx, graphics::Color::BLACK);

        let dark_cyan: graphics::Color = graphics::Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 0.5
        };
        let dark_blue: graphics::Color = graphics::Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 0.25
        };

        let colors: [graphics::Color; 2] = [dark_cyan, dark_blue];
        let selected_colors: [graphics::Color; 2] = [graphics::Color::CYAN, graphics::Color::BLUE];

        for square in 0..64 {
            let color: usize = if (((square % 8) % 2) + (square / 8)) % 2 == 0 { 0 } else { 1 };
            let curr_square = graphics::Rect::new((square % 8 * square_size as u32) as f32, (525 - (square / 8 * square_y as u32)) as f32, square_size, square_y);

            if !self.selected_squares[square as usize] {
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, colors[color])?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default());
            } else {
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, selected_colors[color as usize])?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default());
            }

        }

        let img = graphics::Image::new(ctx, "/white_pawn.png")?;
        graphics::draw(
            ctx,
            &img,
            graphics::DrawParam::default()
            .dest(mint::Point2 {
                x: 0.0, 
                y: 450.0
            }).scale(mint::Vector2 {
                x: 1.0,
                y: 0.75
            })
        )?;    

        graphics::present(ctx)?;
        self.needs_refresh = false;
        Ok(())
    }
}