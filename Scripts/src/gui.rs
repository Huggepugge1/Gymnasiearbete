use ggez;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::input::mouse::MouseButton;
use ggez::graphics;
use ggez::mint;

use std::collections::HashMap;
use ggez::graphics::Image;

use crate::{board, moves};
use crate::board::get_piece;

const SQUARE_SIZE: f32 = 100.0;
const SQUARE_Y: f32 = 0.75 * SQUARE_SIZE;

use crate::MainState;
use crate::ai;

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        if self.frame < 2 {
            self.needs_refresh = true;
        } else {
            self.needs_refresh = false;
        }

        let board_copy: Box<board::Board> = board::copy_board(&self.board);

        if ai::generate_moves(&self.board).len() == 0 {
            if self.board.turn == board::BLACK {
                println!("White has won!");
            } else {
                println!("Black has won!")
            }
            event::quit(ctx);
            return Ok(());
        }

        if self.board.turn == board::BLACK && self.difficulty == ai::EASY {
            let (start_square, end_square) = ai::random_move(&self.board);
            self.board = moves::make_move(board_copy, start_square, end_square);
            self.number_of_selected_squares = 0;
            self.selected_squares = [false; 64];
            self.needs_refresh = true;
            self.frame = 0;
            self.start_square = -1;
            self.end_square = -1;
        } else if self.number_of_selected_squares == 2 {
            self.board = moves::make_move(board_copy, self.start_square, self.end_square);
            self.number_of_selected_squares = 0;
            self.selected_squares = [false; 64];
            self.needs_refresh = true;
            self.start_square = -1;
            self.end_square = -1;
        }
        Ok(())
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

        let piece_link: HashMap<u8, &str> = HashMap::from([
            (board::PAWN, "pawn"),
            (board::ROOK, "rook"),
            (board::KNIGHT, "knight"),
            (board::BISHOP, "bishop"),
            (board::QUEEN, "queen"),
            (board::KING, "king"),

            (board::WHITE, "white"),
            (board::BLACK, "black")
        ]);

        let colors: [graphics::Color; 2] = [dark_cyan, dark_blue];
        let selected_colors: [graphics::Color; 2] = [graphics::Color::CYAN, graphics::Color::BLUE];

        for square in 0..64 {
            let color: usize = if (((square % 8) % 2) + (square / 8)) % 2 == 0 { 0 } else { 1 };
            let x: f32 = (square % 8 * SQUARE_SIZE as u32) as f32;
            let y: f32 = (525 - square / 8 * SQUARE_Y as u32) as f32;
            let curr_square = graphics::Rect::new(x, y, SQUARE_SIZE, SQUARE_Y);

            let piece: board::Piece = get_piece(&self.board, square as i8);

            if !self.selected_squares[square as usize] {
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, colors[color])?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default())?;
            } else {
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, selected_colors[color as usize])?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default())?;
            }


            if piece.piece_type != board::EMPTY {
                let path: String = format!("/{0}_{1}.png", piece_link[&piece.color], piece_link[&piece.piece_type]);
                let img: Image = graphics::Image::new(ctx, path)?;

                graphics::draw(
                    ctx,
                    &img,
                    graphics::DrawParam::default()
                        .dest(mint::Point2 {
                            x,
                            y
                        }).scale(mint::Vector2 {
                        x: 1.0,
                        y: 0.75
                    })
                )?;
            }
            if 1 << square & self.board.current_move > 0 {
                let x: f32 = x + 50.0;
                let y: f32 = y + 50.0 * 0.75;
                let elliplse = graphics::Mesh::new_ellipse (
                    ctx,
                    graphics::DrawMode::fill(),
                    mint::Point2 {
                        x,
                        y
                    },
                    15.0,
                    11.25,
                    0.001,
                    graphics::Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 0.75
                    },
                )?;
                graphics::draw(ctx, &elliplse, graphics::DrawParam::default())?;
            }
        }


        if self.board.promoted != -1 {
            let x: f32 = 200.0;
            let y: f32 = 300.0;
            let pieces: [u8;4] = [board::ROOK, board::KNIGHT, board::BISHOP, board::QUEEN];
            let color: u8 = self.board.turn;
            for i in 0..4 {
                let curr_square = graphics::Rect::new(x + i as f32 * 100.0, y, SQUARE_SIZE, SQUARE_Y);
                let curr_square_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), curr_square, graphics::Color::WHITE)?;
                graphics::draw(ctx, &curr_square_mesh, graphics::DrawParam::default())?;

                let x: f32 = x + i as f32 * 100.0;
                let path: String = format!("/{0}_{1}.png", piece_link[&color], piece_link[&pieces[i]]);
                let img: Image = graphics::Image::new(ctx, path)?;

                graphics::draw(
                    ctx,
                    &img,
                    graphics::DrawParam::default()
                        .dest(mint::Point2 {
                            x,
                            y
                        }).scale(mint::Vector2 {
                        x: 1.0,
                        y: 0.75
                    })
                )?;
            }
        }

        self.frame += 1;
        graphics::present(ctx)?;
        self.needs_refresh = false;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32) {
        self.frame -= 1;
        let square: i8 = (((_x / SQUARE_SIZE) as u32 % 8) + ((((800.0 - _y) / SQUARE_SIZE) as u32 % 8) * 8)) as i8;
        if self.board.promoted == -1 {
            if self.selected_squares[square as usize] == false && self.number_of_selected_squares < 2 {
                if self.number_of_selected_squares == 0 {
                    self.start_square = square;
                    self.board.current_move = moves::get_legal_moves(&self.board, square);
                } else {
                    self.end_square = square;
                    self.board.current_move = 0;
                }

                self.selected_squares[square as usize] = true;
                self.number_of_selected_squares += 1;

            } else if self.selected_squares[square as usize] == true {
                self.selected_squares[square as usize] = false;
                self.number_of_selected_squares -= 1;

                if self.number_of_selected_squares == 0 {
                    self.start_square = -1;
                    self.board.current_move = 0;
                } else {
                    if self.start_square == square {
                        self.start_square = self.end_square;
                        self.end_square = -1;
                    } else {
                        self.end_square = -1;
                    }
                }
            }
        } else {
            if square > 25 && square < 30 {
                if square == 26 {
                    self.board.promoted_piece = board::ROOK;
                } else if square == 27 {
                    self.board.promoted_piece = board::KNIGHT;
                } else if square == 28 {
                    self.board.promoted_piece = board::BISHOP;
                } else if square == 29 {
                    self.board.promoted_piece = board::QUEEN;
                }
                self.board = moves::promote_piece(board::copy_board(&self.board))
            }
        }
    }
}
