#![windows_subsystem = "windows"]

use ::rand::prelude::*;
use macroquad::prelude::*;

struct MainState {}

struct Field {
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { alive: false }
    }

    pub fn update(&mut self, alive: bool) {
        self.alive = alive;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "cells".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    macroquad::window::screen_height();

    let state = MainState {};
    let mut rng = thread_rng();

    const SIZE: Vec2 = Vec2 { x: 100., y: 100. };
    let mut cell_vec = Vec::with_capacity(SIZE.x as usize);
    for x in 0..(SIZE.x as i32) {
        cell_vec.push(Vec::with_capacity(SIZE.y as usize));
    }
    for x in 0..(SIZE.x as i32) {
        for y in 0..(SIZE.y as i32) {
            let mut new_cell = Cell::new();
            new_cell.alive = rng.gen_bool(0.5);
            cell_vec[x as usize].push(new_cell);
        }
    }

    let mut field = Field {
        size: Vec2 { x: 100., y: 100. },
        cells: cell_vec,
    };

    // GameLoop
    loop {
        clear_background(BLACK);

        let mut x_pos = 0;
        let im_cells = field.cells.to_vec();
        for arr in &mut field.cells {
            let mut y_pos = 0;
            for cell in arr {
                let mut alive_count: i8 = 0;
                for x_n in -1..=1 as i32 {
                    for y_n in -1..=1 as i32 {
                        if (x_pos + x_n >= 0 && x_pos + x_n <= 99)
                            && (y_pos + y_n >= 0 && y_pos + y_n <= 99)
                            && (Vec2 {
                                x: (x_pos + x_n) as f32,
                                y: (y_pos + y_n) as f32,
                            } != Vec2 {
                                x: x_pos as f32,
                                y: y_pos as f32,
                            })
                            && im_cells[(x_pos + x_n) as usize][(y_pos + y_n) as usize].alive
                        {
                            alive_count += 1;
                        }
                    }
                }

                if cell.alive {
                    if alive_count > 1 && alive_count < 4 {
                        cell.update(true);
                    } else {
                        cell.update(false);
                    }
                } else if alive_count == 3 {
                    cell.update(true);
                }

                let real_pos = Vec2 {
                    x: (x_pos * 8 + 4) as f32,
                    y: (y_pos * 8 + 4) as f32,
                };
                let mouse_pos = Vec2 {
                    x: mouse_position().0,
                    y: mouse_position().1,
                };
                if is_mouse_button_down(MouseButton::Left) && real_pos.distance(mouse_pos) < 32. {
                    cell.update(rng.gen_bool(0.5));
                }

                draw_rectangle(
                    x_pos as f32 * 8.,
                    y_pos as f32 * 8.,
                    8.,
                    8.,
                    if cell.alive { WHITE } else { BLACK },
                );
                y_pos += 1;
            }
            x_pos += 1;
        }

        next_frame().await;
    }
}
