use macroquad::prelude::*;

#[macroquad::main("macroquad_test")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
