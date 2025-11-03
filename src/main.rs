mod materials;
mod ui;

use crate::{materials::*, ui::*};
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

    let gradient_material = load_material(
        ShaderSource::Glsl {
            vertex: GRADIENT_VERTEX_SHADER,
            fragment: GRADIENT_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("StartColor", UniformType::Float4),
                UniformDesc::new("EndColor", UniformType::Float4),
            ],
            ..Default::default()
        },
    ).unwrap();

    gradient_material.set_uniform("StartColor", Vec4::new(0.1, 0.1, 0.5, 1.0));
    gradient_material.set_uniform("EndColor", Vec4::new(0.5, 0.1, 0.1, 1.0));

    let blur_material = load_material(
        ShaderSource::Glsl {
            vertex: BLUR_VERTEX_SHADER,
            fragment: BLUR_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Color", UniformType::Float4),
                UniformDesc::new("Radius", UniformType::Float1),
                UniformDesc::new("TexelSize", UniformType::Float2),
            ],
            ..Default::default()
        },
    ).unwrap();

    blur_material.set_uniform("Radius", 3. as f32);
    blur_material.set_uniform("TexelSize", 1. / Vec2::new(screen_width(), screen_height()));

    let gray = Color::from_vec(Vec4::new(0.3, 0.3, 0.3, 0.5));

    let mut container = Container {
        outer_margin: 0.01,
        inner_margin: 0.01,
        containers: vec![
            Container {
                components: vec![
                    Box::new(RoundedPanel {
                        material: Some(blur_material.clone()),
                        color: gray,
                        radius: 0.025,
                        ..Default::default()
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
                                material: Some(blur_material.clone()),
                                color: gray,
                                radius: 0.025,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                    Container {
                        components: vec![
                            Box::new(RoundedPanel {
                                material: Some(blur_material.clone()),
                                color: gray,
                                radius: 0.025,
                                ..Default::default()
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

        gl_use_material(&gradient_material);
        draw_rectangle(0., 0., screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        container.draw();

        next_frame().await;
    }
}
