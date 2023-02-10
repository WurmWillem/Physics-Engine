use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{rigid_body::RigidBody, METRE_IN_PIXELS, SCREEN_SIZE, SCREEN_SIZE_METRES};

pub struct Engine {
    rigid_bodies: Vec<RigidBody>,
    g: f32,
    c: f32,
    pause: bool,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            rigid_bodies: vec![RigidBody::new(90.), RigidBody::new(0.1)],
            pause: false,
            g: 0.,
            c: 1.,
        }
    }
    pub fn update(&mut self) {
        self.update_ui();
        if !self.pause {
            for rb in &mut self.rigid_bodies {
                rb.apply_forces(self.g, self.c);
            }
        }
    }
    pub fn draw(&self) {
        draw_background();
        for rb in &self.rigid_bodies {
            rb.draw();
        }
    }

    fn update_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.heading("General");
                ui.label(format!("FPS: {}", get_fps()));
                ui.label(format!("World size: {} m", SCREEN_SIZE_METRES));
                ui.horizontal(|ui| {
                    if ui.button("Reset all").clicked() {
                        *self = Engine::new();
                    }
                    if ui.button("Pause").clicked() {
                        self.pause = !self.pause;
                    }
                });
                ui.separator();

                ui.heading("Variables");
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

                ui.horizontal(|ui| {
                    ui.label("c:");
                    ui.add(egui::Slider::new(&mut self.c, (-1.)..=30.));
                    if ui.button("Reset to default").clicked() {
                        self.c = 1.;
                    }
                    if ui.button("Reset to 0").clicked() {
                        self.c = 0.;
                    }
                });
            });
            
            for i in 0..self.rigid_bodies.len() {
                self.rigid_bodies[i].update_ui(egui_ctx, i + 1);
            }
        });
    }
}

fn draw_background() {
    for x in 0..=(SCREEN_SIZE_METRES.x as usize) {
        draw_line(
            x as f32 * METRE_IN_PIXELS.x,
            0.,
            x as f32 * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y,
            0.5,
            BLACK,
        )
    }
    for y in 0..=(SCREEN_SIZE_METRES.y as usize) {
        draw_line(
            0.,
            SCREEN_SIZE.y - y as f32 * METRE_IN_PIXELS.y,
            SCREEN_SIZE.x,
            SCREEN_SIZE.y - y as f32 * METRE_IN_PIXELS.y,
            0.5,
            BLACK,
        )
    }
    draw_line(
        0.,
        SCREEN_SIZE.y,
        SCREEN_SIZE.x,
        SCREEN_SIZE.y,
        METRE_IN_PIXELS.y * 2.,
        BROWN,
    )
}
