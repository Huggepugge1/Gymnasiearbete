use ggez;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::input::mouse::MouseButton;
use ggez::graphics;
use ggez::mint::Point2;

const square_size: f32 = 100.0;
const square_y: f32 = 0.75 * square_size;

struct MainState {
    pub selected_squares: [bool; 64],
    pub number_of_selected_squares: u8
}

impl MainState {
    pub fn new() -> Self {
        MainState {
            selected_squares: [false; 64],
            number_of_selected_squares: 0
        }
    }
}

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
        println!("{}", (_y));
        let square: usize = (((_x / square_size) as u32 % 8) + (((_y / square_size) as u32 % 8) * 8)) as usize;
        if self.selected_squares[square] == false && self.number_of_selected_squares < 2 {
            self.selected_squares[square] = true;
            self.number_of_selected_squares += 1;
        } else if self.selected_squares[square] == true {
            self.selected_squares[square] = false;
            self.number_of_selected_squares -= 1;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
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
            let curr_square = graphics::Rect::new(((square % 8 * square_size as u32)) as f32, ((square / 8 * square_y as u32)) as f32, square_size, square_y);

            if !self.selected_squares[square as usize] {
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, colors[color])?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default());
            } else {
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, selected_colors[color as usize])?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default());
            }

        }

        graphics::present(ctx)?;
        Ok(())
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
