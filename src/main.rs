mod cell;
mod board;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use crate::board::Board;

const GRID_WIDTH: i32 = 48;
const GRID_HEIGHT: i32 = 48;
const CELL_SIZE: i32 = 12;

const SCREEN_WIDTH: i32 = CELL_SIZE * GRID_WIDTH;
const SCREEN_HEIGHT: i32 = CELL_SIZE * GRID_HEIGHT;

const FRAME_RATE: u32 = 60;

const NEXT_GEN_INTERVAL_INITIAL: f32 = 1.0 / 6.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game of Life")
        .build();

    rl.set_target_fps(FRAME_RATE);

    let camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut board: Board = Board::init_board();

    let mut frame = 0;
    let mut pause: bool = true;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }

        if rl.is_key_pressed(KEY_R) {
            drop(board); // not sure if i need it, in the C version i freed the cells and the board before initializing a new one
            board = Board::init_board();
        }

        if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) || rl.is_mouse_button_down(MOUSE_RIGHT_BUTTON) {
            let x: i32 = rl.get_mouse_x() / CELL_SIZE;
            let y: i32 = rl.get_mouse_y() / CELL_SIZE;

            if !(x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT) {
                if rl.is_mouse_button_down(MOUSE_RIGHT_BUTTON) {
                    // kill cell that was clicked on
                    board.cells[((y * GRID_WIDTH) + x) as usize].alive = false;
                    board.set_next_status(x, y, false);
                } else {
                    // paint cells with left mouse button
                    board.cells[((y * GRID_WIDTH) + x) as usize].alive = true;
                }
            }
        }


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d2 = d.begin_mode2D(camera);
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    if board.check_status(x, y) {
                        d2.draw_rectangle(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, Color::WHITE);
                    }
                    d2.draw_rectangle_lines(x * CELL_SIZE, y * CELL_SIZE, 1, 1, Color::GRAY);
                }
            }
        }
        // end mode 2D

        d.draw_fps(SCREEN_WIDTH - 80, 0);

        if pause {
            d.draw_text("Paused", 2, 2, 40, Color::RED);
        } else {
            if frame == (FRAME_RATE as f32 * NEXT_GEN_INTERVAL_INITIAL) as i32 {
                board.play_round();
                frame = 1;
            }
            frame += 1;
        }
    }

}