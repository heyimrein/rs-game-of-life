use macroquad::prelude::*;

struct MainState {}

#[macroquad::main("pills")]
async fn main() {
    let state = MainState {};

    loop {
        clear_background(BLACK);
        draw_circle(25., 25., 25., WHITE);

        next_frame().await;
    }
}
