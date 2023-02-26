use crate::engine::Variables;
use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{
    rigid_body::{Forces, Format, RigidBodies, RigidBody},
    SCREEN_SIZE,
};

pub const WORLD_SIZE: Vec2 = vec2(40., 35.);
pub const METRE_IN_PIXELS: Vec2 = vec2(SCREEN_SIZE.x / WORLD_SIZE.x, SCREEN_SIZE.y / WORLD_SIZE.y);

#[derive(Debug, Clone, Copy)]
pub struct BouncingBall {
    enabled: bool,
    mass: f32,
    radius: f32,
    pos: Vec2,
    vel: Vec2,
    forces: Forces,
    default_pos: Vec2,
    default_mass: f32,
}
impl BouncingBall {
    pub fn new(mass: f32, pos: Vec2, radius: f32) -> Self {
        let forces = Forces::new(true, true);
        Self {
            enabled: true,
            mass,
            radius,
            pos,
            vel: Vec2::ZERO,
            forces,
            default_pos: pos,
            default_mass: mass,
        }
    }
}
impl RigidBody for BouncingBall {
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
        let f_air = c * self.radius * 2. * self.vel * self.vel.abs();
        f_res -= f_air;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_t;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_t;

        if next_pos.y + self.radius > WORLD_SIZE.y {
            self.vel.y *= -1.;
            self.pos.y = WORLD_SIZE.y - self.radius;
        } else if next_pos.y - self.radius - 1. < 0. {
            self.vel.y *= -1.;
            self.pos.y = self.radius + 1.;
        } else {
            self.pos = next_pos;
        }
        self.forces.f_res = f_res;
        self.forces.f_g = Some(f_g);
        self.forces.f_air = Some(f_air);
    }
    fn draw(&self) {
        draw_circle(
            self.pos.x * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRE_IN_PIXELS.y,
            self.radius * METRE_IN_PIXELS.x,
            RED,
        )
    }
    fn update_ui(&mut self, egui_ctx: &Context, index: usize) {
        egui::Window::new(format!("Bouncing ball {index}")).show(egui_ctx, |ui| {
            ui.set_max_width(200.);
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.enabled, "enabled");
                if ui.button("Reset all").clicked() {
                    *self = BouncingBall::new(self.default_mass, self.default_pos, self.radius);
                }
            });

            ui.collapsing("Show data", |ui| {
                ui.heading("Data");
                ui.horizontal(|ui| {
                    ui.label("Mass:");
                    ui.add(egui::Slider::new(&mut self.mass, (0.1)..=30.));
                    ui.label("kg");
                });

                ui.label(format!("Radius: {} m", self.radius));
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
            });
            self.forces.display_ui(ui);
        });
    }
    fn get_enabled(&self) -> bool {
        self.enabled
    }
}