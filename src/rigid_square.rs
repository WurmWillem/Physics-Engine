use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{
    engine::Variables,
    rigid_body::{Forces, RigidBody, RigidBodyType},
    SCREEN_SIZE,
};
use macroquad::math::Vec2;

pub const WORLD_SIZE: Vec2 = vec2(60., 52.5);
pub const METRE_IN_PIXELS: Vec2 = vec2(SCREEN_SIZE.x / WORLD_SIZE.x, SCREEN_SIZE.y / WORLD_SIZE.y);

#[derive(Debug, Clone, Copy)]
pub struct RigidSquare {
    enabled: bool,
    mass: f32,
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    forces: Forces,
    default_pos: Vec2,
    default_mass: f32,
}
impl RigidSquare {
    pub fn new(mass: f32, pos: Vec2, size: Vec2) -> Self {
        let forces = Forces::new(true, true, false);
        Self {
            mass,
            pos,
            vel: Vec2::ZERO,
            size,
            enabled: true,
            forces,
            default_pos: pos,
            default_mass: mass,
        }
    }
}
impl RigidBody for RigidSquare {
    fn apply_forces(&mut self, vars: Variables, delta_time: f32) {
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
        self.vel += acc * delta_time;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_time;

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
        self.forces.f_res = f_res;
        self.forces.f_g = Some(f_g);
        self.forces.f_air = Some(f_air);
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

    fn update_based_on_ui(&mut self, egui_ctx: &Context, index: usize) {
        egui::Window::new(format!("Rigidbody {index}")).show(egui_ctx, |ui| {
            ui.set_max_width(200.);
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.enabled, "enabled");
                if ui.button("Reset all").clicked() {
                    *self = RigidSquare::new(self.default_mass, self.default_pos, self.size);
                }
            });

            ui.collapsing("Show data", |ui| {
                ui.label(format!("Size: {} m", self.size));

                let mut mass_copy = self.mass;
                self.update_default_properties_ui(ui, &mut mass_copy, self.default_pos);
                self.mass = mass_copy;
            });
            self.forces.display_ui(ui);
        });
    }
    fn get_type(&self) -> RigidBodyType {
        RigidBodyType::Square
    }
    fn get_enabled(&self) -> bool {
        self.enabled
    }
    fn get_pos(&self) -> Vec2 {
        self.pos
    }
    fn get_vel(&self) -> Vec2 {
        self.vel
    }
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn get_radius(&self) -> f32 {
        panic!("rigid_square does not have property radius")
    }
    fn set_vel(&mut self, new_vel: Vec2) {
        self.vel = new_vel;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
}
