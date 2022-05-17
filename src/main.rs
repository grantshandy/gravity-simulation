use ::glam::Vec2;
use ::rand::{prelude::ThreadRng, thread_rng, Rng};
use egui_macroquad::{
    egui::{panel::Side, SidePanel},
    macroquad::{self, prelude::*},
};

const SPEED: f32 = 0.035;


struct Node {
    pub location: Vec2,
    pub color: Color,
    pub mass: f32,
}

#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut rng = thread_rng();
    let mut state: Vec<Node> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let mut current_radius: f32 = 10.0;
    let mut currently_selected: Option<usize> = None;
    let mut playing = false;

    loop {
        clear_background(DARKGRAY);

        if playing {
            calc_physx(&mut state);
        }

        if is_key_down(KeyCode::R) {
            state = Vec::new();
        }

        let (mouse_x, mouse_y) = mouse_position();
        if is_key_down(KeyCode::LeftShift) {
            let mouse_wheel_y = mouse_wheel().1;

            if mouse_wheel_y < 0. {
                current_radius -= 1.0;
                if current_radius < 5.0 {
                    current_radius = 5.0;
                }
            }

            if mouse_wheel_y > 0. {
                current_radius += 1.0;
            }

            let mut overlaps = false;
            let mouse_position = Vec2::new(mouse_x, mouse_y);

            for (index, node) in state.iter().enumerate() {
                if node.location.distance(mouse_position) < (node.mass + &current_radius) {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                draw_circle(
                    mouse_position.x,
                    mouse_position.y,
                    current_radius,
                    Color::new(1.0, 1.0, 1.0, 0.5),
                );

                if is_mouse_button_down(MouseButton::Left) {
                    indices.push(indices.len() + 1);

                    state.push(Node {
                        location: mouse_position,
                        color: random_color(&mut rng),
                        mass: current_radius,
                    });
                }
            }
        } else {
            if is_mouse_button_down(MouseButton::Left) {
                let mouse = mouse_position();
                let mouse_position = Vec2::new(mouse.0, mouse.1);
    
                let mut clicked_on_body = false;

                for (index, node) in state.iter().enumerate() {
                    if node.location.distance(mouse_position) < node.mass {
                        currently_selected = Some(index);
                        clicked_on_body = true;
                        break;
                    }
                }

                if !clicked_on_body {
                    currently_selected = None;
                }
            }
        }

        for (index, node) in state.iter().enumerate() {
            let color = match currently_selected {
                Some(selected_index) => {
                    if selected_index == index {
                        RED
                    } else {
                        node.color
                    }
                }
                None => node.color
            };

            draw_circle(node.location.x, node.location.y, node.mass, node.color);
        }

        egui_macroquad::ui(|ctx| {
            SidePanel::new(Side::Left, "side_panel").show(ctx, |ui| {
                let playing_text = match playing {
                    true => "Pause",
                    false => "Play",
                };
    
                if ui.button(playing_text).clicked() {
                    if playing {
                        playing = false;
                    } else {
                        playing = true;
                    }
                }
            });
        });
    
        egui_macroquad::draw();

        next_frame().await
    }
}

fn calc_physx(state: &mut Vec<Node>) {
    for (index, node) in &mut state.iter().enumerate() {
        node.location.y += 1.0;
    }
}

fn random_color(rng: &mut ThreadRng) -> Color {
    Color::from_rgba(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        255,
    )
}