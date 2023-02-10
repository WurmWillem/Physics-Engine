use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{METRE_IN_PIXELS, SCREEN_SIZE, SCREEN_SIZE_METRES};

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub enabled: bool,
    mass: f32,
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    f_res: Vec2,
    f_g: f32,
    f_air: Vec2,
    default_pos: Vec2,
    default_mass: f32,
}
impl RigidBody {
    pub fn new(mass: f32, pos: Vec2, size: Vec2) -> Self {
        Self {
            mass,
            pos,
            vel: Vec2::ZERO,
            size,
            enabled: true,
            f_res: Vec2::ZERO,
            f_g: 0.,
            f_air: Vec2::ZERO,
            default_pos: pos,
            default_mass: mass,
        }
    }
    pub fn apply_forces(&mut self, g: f32, c: f32, time_mult: f32) {
        let delta_t = get_frame_time();

        let mut f_res = Vec2::ZERO;

        //Fz = m * g
        let f_g = g * self.mass;
        f_res.y -= f_g;

        //F_Air = 0.5 * p * A * v*v = k * v*v in our case because k = 0.5 * p * A
        let f_air = c * self.vel * self.vel.abs();
        f_res -= f_air;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_t * time_mult;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_t;

        if next_pos.y > SCREEN_SIZE_METRES.y {
            self.vel.y = 0.;
            self.pos.y = SCREEN_SIZE_METRES.y;
            f_res = Vec2::ZERO;
        } else if next_pos.y - self.size.y < 1. {
            self.vel.y = 0.;
            self.pos.y = self.size.y + 1.;
            f_res = Vec2::ZERO;
        } else {
            self.pos = next_pos;
        }
        self.f_res = f_res;
        self.f_g = f_g;
        self.f_air = f_air
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRE_IN_PIXELS.y,
            self.size.x * METRE_IN_PIXELS.x,
            self.size.y * METRE_IN_PIXELS.y,
            RED,
        );
    }

    pub fn update_ui(&mut self, egui_ctx: &Context, index: usize) {
        egui::Window::new(format!("Rigidbody {index}")).show(egui_ctx, |ui| {
            ui.set_max_width(200.);
            ui.checkbox(&mut self.enabled, "enabled");

            ui.collapsing("Show", |ui| {
                ui.heading("Data");
                ui.horizontal(|ui| {
                    ui.label("Mass:");
                    ui.add(egui::Slider::new(&mut self.mass, (0.1)..=300.));
                    ui.label("kg");
                });

                ui.label(format!("Size: {} m", self.size));
                ui.horizontal(|ui| {
                    ui.label(format!("Velocity: {} m/s", vec2_formatted(self.vel)));
                    if ui.button("Reset").clicked() {
                        self.vel = Vec2::ZERO;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Position: {} m", vec2_formatted(self.pos)));
                    if ui.button("Reset").clicked() {
                        self.pos = self.default_pos;
                    }
                });
                if ui.button("Reset all").clicked() {
                    *self = RigidBody::new(self.default_mass, self.default_pos, self.size);
                }
                ui.separator();

                ui.heading("Forces");
                ui.label(format!(
                    "F_res = {} = {} N",
                    vec2_formatted(self.f_res),
                    f32_formatted(self.f_res.length())
                ));
                ui.label(format!("Gravity: m * g = {} N", self.f_g));
                ui.label("Air resistance: c * v*v =");
                ui.label(format!(
                    "{} = {} N",
                    vec2_formatted(self.f_air),
                    f32_formatted(self.f_air.length())
                ));
            });
        });
    }
}

fn vec2_formatted(vec: Vec2) -> Vec2 {
    let v = vec * 100.;
    let x = v.x as i32 as f32 / 100.;
    let y = v.y as i32 as f32 / 100.;
    Vec2::new(x, y)
}

fn f32_formatted(f: f32) -> f32 {
    let f = f * 100.;
    let f = f as i32 as f32;
    f / 100.
}
