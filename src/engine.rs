use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{rigid_body::RigidBody, SCREEN_SIZE_METRES};

pub struct Engine {
    rb: RigidBody,
    g: i32,
    k: f32,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            rb: RigidBody::new(90.),
            g: 0,
            k: 1.,
        }
    }
    pub fn update(&mut self) {
        self.update_ui();
        self.rb.apply_forces(self.g, self.k);
    }
    pub fn draw(&self) {
        self.rb.draw();
    }

    fn update_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.label(format!("FPS: {}", get_fps()));
                ui.label(format!("World size: {} m", SCREEN_SIZE_METRES));
                ui.separator();

                ui.heading("Rigidbody");
                ui.label(format!("Mass: {} kg", self.rb.mass));
                ui.label(format!("Size: {} m", self.rb.size));
                ui.horizontal(|ui| {
                    ui.label(format!("Velocity: {} m/s", self.rb.vel));
                    if ui.button("Reset").clicked() {
                        self.rb.vel = Vec2::new(0., 0.);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("Position: {} m", self.rb.pos));
                    if ui.button("Reset").clicked() {
                        self.rb.pos =
                            Vec2::new(SCREEN_SIZE_METRES.x * 0.5, SCREEN_SIZE_METRES.y * 0.5);
                    }
                });

                ui.separator();

                ui.heading("Forces");
                ui.label("Gravity: m * g");
                ui.horizontal(|ui| {
                    ui.label("g:");
                    ui.add(egui::Slider::new(&mut self.g, -30..=30));
                    if ui.button("Reset").clicked() {
                        self.g = 0;
                    }
                });
                ui.label("Air resistance: k * v*v");
                ui.horizontal(|ui| {
                    ui.label("k: ");
                    ui.add(egui::Slider::new(&mut self.k, (-1.)..=10.));
                    if ui.button("Reset").clicked() {
                        self.k = 0.;
                    }
                });
            });
        });
    }
}
