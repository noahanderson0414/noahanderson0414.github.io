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
        outer_margin: 0.01,
        inner_margin: 0.01,
        components: vec![
            Box::new(Panel {
                color: Color::from_vec(Vec4::new(0.1, 0.1, 0.1, 1.)),
            }),
        ],
        containers: vec![
            Container {
                weight: 2.,
                components: vec![
                    Box::new(RoundedPanel {
                        color: RED,
                        radius: 0.025,
                    }),
                ],
                ..Default::default()
            },
            Container {
                inner_margin: 0.01,
                weight: 3.,
                align: Align::Vertical,
                containers: vec![
                    Container {
                        weight: 2.,
                        components: vec![
                            Box::new(RoundedPanel {
                                color: BLUE,
                                radius: 0.025,
                            }),
                        ],
                        ..Default::default()
                    },
                    Container {
                        components: vec![
                            Box::new(RoundedPanel {
                                color: GREEN,
                                radius: 0.025,
                            }),
                        ],
                        ..Default::default()
                    },
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
