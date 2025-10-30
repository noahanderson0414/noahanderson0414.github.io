mod ui;

use crate::ui::*;
use macroquad::prelude::*;

#[macroquad::main("macroquad_test")]
async fn main() {
    if cfg!(target_arch = "wasm32") {
        loop {
            clear_background(BLACK);

            draw_text("WORK IN PROGRESS", 0., 0., 12., WHITE);

            next_frame().await
        }
    }

    let left_rectangle = RoundedRectangle {
        position: Vec2::splat(0.025),
        size: Vec2::new(0.4625, 0.95),
        radius: 0.1,
        sides: 32,
        anchor: Anchor::TopLeft,
        color: Color::from_rgba(192, 64, 64, 255),
    };

    let right_rectangle = RoundedRectangle {
        position: Vec2::new(0.975, 0.025),
        size: Vec2::new(0.4625, 0.95),
        radius: 0.1,
        sides: 32,
        anchor: Anchor::TopRight,
        color: Color::from_rgba(64, 64, 192, 255),
    };

    loop {
        clear_background(BLACK);

        left_rectangle.draw();
        right_rectangle.draw();

        next_frame().await;
    }
}
