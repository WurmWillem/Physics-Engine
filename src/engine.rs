use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{rigid_body::RigidBody, METRE_IN_PIXELS, SCREEN_SIZE, SCREEN_SIZE_METRES};

pub struct Engine {
    rb: RigidBody,
    g: f32,
    k: f32,
    square_tex: Texture2D,
}
impl Engine {
    pub fn new(square_tex: Texture2D) -> Self {
        Self {
            rb: RigidBody::new(90.),
            g: 0.,
            k: 1.,
            square_tex,
        }
    }
    pub fn update(&mut self) {
        self.update_ui();
        self.rb.apply_forces(self.g, self.k);
    }
    pub fn draw(&self) {
        draw_background();
        self.rb.draw();
    }

    fn update_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.heading("General");
                ui.label(format!("FPS: {}", get_fps()));
                ui.label(format!("World size: {} m", SCREEN_SIZE_METRES));
                if ui.button("Reset all").clicked() {
                    *self = Engine::new(self.square_tex);
                }
                ui.separator();

                ui.heading("Forces");
                ui.label(format!("F_res = {}", vec2_formatted(self.rb.f_res)));
                ui.label(format!("Gravity: m * g = {} N", self.rb.f_g));
                ui.horizontal(|ui| {
                    ui.label("g:");
                    ui.add(egui::Slider::new(&mut self.g, (-30.)..=30.));
                    if ui.button("Reset to default").clicked() {
                        self.g = 9.81;
                    }
                    if ui.button("Reset to 0").clicked() {
                        self.g = 0.;
                    }
                });

                ui.label(format!(
                    "Air resistance: k * v*v = {}",
                    vec2_formatted(self.rb.f_air)
                ));
                ui.horizontal(|ui| {
                    ui.label("k:");
                    ui.add(egui::Slider::new(&mut self.k, (-1.)..=30.));
                    if ui.button("Reset to default").clicked() {
                        self.k = 1.;
                    }
                    if ui.button("Reset to 0").clicked() {
                        self.k = 0.;
                    }
                });
            });
            self.rb.update_ui(egui_ctx);
        });
    }
}

fn draw_background() {
    for x in 0..(SCREEN_SIZE_METRES.x as usize) {
        draw_line(
            x as f32 * METRE_IN_PIXELS.x,
            0.,
            x as f32 * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y,
            1.,
            BLACK,
        )
    }
    for y in 0..(SCREEN_SIZE_METRES.y as usize) {
        draw_line(
            0.,
            y as f32 * METRE_IN_PIXELS.y,
            SCREEN_SIZE.x,
            y as f32 * METRE_IN_PIXELS.y,
            1.,
            BLACK,
        )
    }
}

pub fn vec2_formatted(vec: Vec2) -> Vec2 {
    let v = vec * 100.;
    let x = v.x as i32 as f32 / 100.;
    let y = v.y as i32 as f32 / 100.;
    Vec2::new(x, y)
}
