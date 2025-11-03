mod ui;

use crate::ui::*;
use macroquad::prelude::*;

#[macroquad::main("macroquad_test")]
async fn main() {
    if cfg!(target_arch = "wasm32") {
        loop {
            clear_background(BLACK);

            if is_key_down(KeyCode::Backspace) && is_key_down(KeyCode::Enter) {
                break;
            }

            draw_text("WORK IN PROGRESS", 16., 48., 64., WHITE);

            next_frame().await
        }
    }

    let mut container = Container {
        containers: vec![
            Container {
                weight: 2.,
                components: vec![
                    Box::new(Panel {
                        color: RED,
                    }),
                ],
                ..Default::default()
            },
            Container {
                components: vec![
                    Box::new(Panel {
                        color: GREEN,
                    }),
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        container.draw();

        next_frame().await;
    }
}
