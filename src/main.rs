use std::f32::consts::PI;

use ::glam::Vec2;
use egui_macroquad::{
    egui::{epaint::Shadow, Align2, Frame, Style, Window},
    macroquad::{self, prelude::*},
};

const GRAVITATIONAL_CONSTANT: f32 = 500.0;

#[derive(Copy, Clone, PartialEq)]
struct Node {
    pub location: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub area: f32,
}

#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut state: Vec<Node> = Vec::new();
    let mut current_radius: f32 = 10.0;
    let mut playing = false;

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_position = Vec2::new(mouse_x, mouse_y);
        let edit_mode = is_key_down(KeyCode::LeftShift);
        let clicking = is_mouse_button_down(MouseButton::Left);

        clear_background(Color::from_rgba(30, 58, 138, 255));
        update_radius(&mut current_radius);

        if playing {
            calc_physx(&mut state);
            calc_overlaps(&mut state);
        }

        if is_key_down(KeyCode::R) {
            state = Vec::new();
        }

        if edit_mode
            && overlaps_at_all(
                (mouse_position, radius_from_area(current_radius) * 10.0),
                &state,
            )
            .is_none()
        {
            draw_circle(
                mouse_position.x,
                mouse_position.y,
                radius_from_area(current_radius * 10.0),
                Color::new(1.0, 1.0, 1.0, 0.5),
            );

            if clicking {
                state.push(Node {
                    location: mouse_position,
                    velocity: Vec2::ZERO,
                    color: WHITE,
                    area: current_radius * 10.0,
                });
            }
        }

        draw(&state);

        egui_macroquad::ui(|ctx| {
            let mut shadow = Shadow::default();
            shadow.extrusion = 0.0;

            Window::new("menu")
                .title_bar(false)
                .anchor(Align2::LEFT_TOP, [20.0, 20.0])
                .resizable(false)
                .frame(Frame::window(&Style::default()).shadow(shadow))
                .show(ctx, |ui| {
                    if ui.button(if playing { "Pause" } else { "Play" }).clicked() {
                        playing = opposite(playing);
                    }
                    if ui.button("Reset").clicked() {
                        state = Vec::new();
                    }
                });

            if !state.is_empty() {
                Window::new("label")
                    .title_bar(false)
                    .anchor(Align2::RIGHT_TOP, [-20.0, 20.0])
                    .resizable(false)
                    .frame(Frame::window(&Style::default()).shadow(shadow))
                    .show(ctx, |ui| {
                        ui.label("Velocities:");
                        for (num, node) in state.iter().enumerate() {
                            ui.label(format!("{num}: [{:.2}, {:.2}]", node.velocity.x, node.velocity.y));
                        }
                    });
            }
        });

        egui_macroquad::draw();

        next_frame().await
    }
}

fn draw(state: &Vec<Node>) {
    for node in state.iter() {
        draw_circle(
            node.location.x,
            node.location.y,
            radius_from_area(node.area),
            node.color,
        );
    }
}

fn update_radius(current_radius: &mut f32) {
    let mouse_wheel_y = mouse_wheel().1;

    if mouse_wheel_y < 0. {
        *current_radius -= 1.0;
        if *current_radius < 5.0 {
            *current_radius = 5.0;
        }
    }

    if mouse_wheel_y > 0. {
        *current_radius += 1.0;
    }
}

fn calc_overlaps(state: &mut Vec<Node>) {
    let state_clone = state.clone();

    for (index, node) in state_clone.iter().enumerate() {
        if let Some(overlap_index) =
            overlaps_at_all((node.location, radius_from_area(node.area)), &state_clone)
        {
            if overlap_index == index {
                continue;
            }

            let node = &mut state[index];
            let overlap_node = &state_clone[overlap_index];

            node.area += overlap_node.area;
            node.velocity =
                (overlap_node.velocity / overlap_node.area) + (node.velocity / node.area);
            node.location = if node.area > overlap_node.area {
                node.location
            } else {
                overlap_node.location
            };

            state.remove(overlap_index);
        }
    }
}

fn calc_physx(state: &mut Vec<Node>) {
    let state_clone = state.clone();

    for index in 0..state_clone.len() {
        let mut final_force = Vec2::ZERO;

        for other_index in 0..state_clone.len() {
            if index == other_index {
                continue;
            }

            let node_one = &state_clone[index];
            let node_two = &state_clone[other_index];

            final_force += -(GRAVITATIONAL_CONSTANT
                * ((node_one.area * node_two.area)
                    / node_one.location.distance_squared(node_two.location)))
                * ((node_one.location - node_two.location)
                    / node_one.location.distance(node_two.location));
        }

        let td = 0.035;

        let node = &mut state[index];
        node.velocity += (final_force / node.area) * td;
        node.location += node.velocity * td;
    }
}

fn overlaps_at_all(node: (Vec2, f32), state: &Vec<Node>) -> Option<usize> {
    let mut o = None;

    for (index, other_node) in state.iter().enumerate() {
        if circles_overlap((node.0, node.1), (other_node.location, other_node.area)) {
            o = Some(index);
        }
    }

    o
}

fn circles_overlap(node_one: (Vec2, f32), node_two: (Vec2, f32)) -> bool {
    if node_one.0.distance(node_two.0)
        < (radius_from_area(node_one.1) + radius_from_area(node_two.1))
    {
        true
    } else {
        false
    }
}

fn opposite(b: bool) -> bool {
    if b {
        false
    } else {
        true
    }
}

fn radius_from_area(area: f32) -> f32 {
    (area / PI).sqrt()
}
