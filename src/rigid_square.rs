use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{
    engine::{RigidBodies, RigidBody},
    scenes::Variables,
    SCREEN_SIZE,
};

pub const WORLD_SIZE: Vec2 = vec2(60., 52.5);
pub const METRE_IN_PIXELS: Vec2 = vec2(SCREEN_SIZE.x / WORLD_SIZE.x, SCREEN_SIZE.y / WORLD_SIZE.y);

const DIGITS_AFTER_DECIMAL: usize = 0;

#[derive(Debug, Clone, Copy)]
pub struct RigidSquare {
    enabled: bool,
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
impl RigidBody for RigidSquare {
    fn apply_forces(&mut self, vars: Variables, time_mult: f32, _rigid_bodies: &RigidBodies) {
        let delta_t = get_frame_time() * time_mult;

        let g = match vars.g {
            Some(g_) => g_,
            None => panic!("g is None"),
        };
        let c = match vars.c {
            Some(c_) => c_,
            None => panic!("c is None"),
        };

        let mut f_res = Vec2::ZERO;

        //Fz = m * g
        let f_g = g * self.mass;
        f_res.y -= f_g;

        //F_Air = 0.5 * p * A * v*v = c * A * v*v in our case because k = 0.5 * p
        let f_air = c * self.size.x * self.vel * self.vel.abs();
        f_res -= f_air;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_t;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_t;

        if next_pos.y > WORLD_SIZE.y {
            self.vel.y = 0.;
            self.pos.y = WORLD_SIZE.y;
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
        self.f_air = f_air;
    }

    fn draw(&self) {
        draw_rectangle(
            self.pos.x * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRE_IN_PIXELS.y,
            self.size.x * METRE_IN_PIXELS.x,
            self.size.y * METRE_IN_PIXELS.y,
            RED,
        );
    }

    fn update_ui(&mut self, egui_ctx: &Context, index: usize) {
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
                    ui.label(format!("Velocity: {} m/s", self.vel.format()));
                    if ui.button("Reset").clicked() {
                        self.vel = Vec2::ZERO;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Position: {} m", self.pos.format()));
                    if ui.button("Reset").clicked() {
                        self.pos = self.default_pos;
                    }
                });
                if ui.button("Reset all").clicked() {
                    *self = RigidSquare::new(self.default_mass, self.default_pos, self.size);
                }
                ui.separator();

                ui.heading("Forces")
                    .on_hover_text("Forces that get applied to the rigidbody");
                ui.label(format!(
                    "F_res = {} = {} N",
                    self.f_res.format(),
                    self.f_res.length().format()
                ));
                ui.label(format!("Gravity: m * g = {} N", self.f_g.format()));
                ui.label("Air resistance: c * A * v*v =");
                ui.label(format!(
                    "{} = {} N",
                    self.f_air.format(),
                    self.f_air.length().format()
                ));
            });
        });
    }

    fn get_enabled(&self) -> bool {
        self.enabled
    }
}
impl RigidSquare {
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
}

trait Format {
    fn format(&self) -> Self;
}
impl Format for f32 {
    fn format(&self) -> Self {
        let f = *self * (DIGITS_AFTER_DECIMAL + 1) as f32;
        let f = f.round();
        f / (DIGITS_AFTER_DECIMAL + 1) as f32
    }
}
impl Format for Vec2 {
    fn format(&self) -> Self {
        vec2(self.x.format(), self.y.format())
    }
}
