use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{
    engine::Variables,
    rigid_body::{Forces, RigidBody, RigidBodyType},
    SCREEN_SIZE,
};
use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct RigidSquare {
    enabled: bool,
    mass: f32,
    restitution: f32,
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
            restitution: 0.4,
            enabled: true,
            forces,
            default_pos: pos,
            default_mass: mass,
        }
    }
}
impl RigidBody for RigidSquare {
    fn apply_forces(&mut self, vars: Variables, delta_time: f32, scene_size: Vec2) {
        let mut f_res = Vec2::ZERO;
        let mut f_g = None;
        let mut f_air = None;

        if let Some(g) = vars.g {
            //Fz = m * g
            let f_gravity = g * self.mass;
            f_res.y -= f_gravity;
            f_g = Some(f_gravity);
        }

        if let Some(c) = vars.c {
            //F_Air = 0.5 * p * A * v*v = c * A * v*v in our case because k = 0.5 * p
            let f_air_resistance = c * self.size.x * self.vel * self.vel.abs();
            f_res -= f_air_resistance;
            f_air = Some(f_air_resistance)
        }

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_time;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_time;

        // Check next pos y for collisions with world corners
        if next_pos.y > scene_size.y {
            self.vel.y *= -self.restitution;
            self.pos.y = scene_size.y;
            f_res.y = 0.;
        } else if next_pos.y - self.size.y < 1. {
            self.vel.y *= -self.restitution;
            self.pos.y = self.size.y + 1.;
            f_res.y = 0.;
        } else {
            self.pos.y = next_pos.y;
        }
        // Check next pos x for collisions with world corners
        if next_pos.x + self.size.x > scene_size.x {
            self.vel.x *= -self.restitution;
            self.pos.x = scene_size.x - self.size.x;
            f_res.x = 0.;
        } else if next_pos.x < 0. {
            self.vel.x *= -self.restitution;
            self.pos.x = 0.;
            f_res.x = 0.;
        } else {
            self.pos.x = next_pos.x;
        }

        /*if next_pos.y + self.size.x > scene_size.y {
            self.vel.y *= -1.;
            self.pos.y = scene_size.y - self.size.y;
        } else if next_pos.y - self.size.y - 1. < 0. {
            self.vel.y *= -1.;
            self.pos.y = self.size.y + 1.;
        } else if next_pos.x + self.size.x > scene_size.x {
            self.vel.x *= -1.;
            self.pos.x = scene_size.x - self.size.x;
        } else if next_pos.x - self.size.x < 0. {
            self.vel.x *= -1.;
            self.pos.x = self.size.x;
        } else {
            self.pos = next_pos;
        }*/

        self.forces.f_res = f_res;
        self.forces.f_g = f_g;
        self.forces.f_air = f_air;
    }

    fn draw(&self, metre_in_pixels: Vec2) {
        draw_rectangle(
            self.pos.x * metre_in_pixels.x,
            SCREEN_SIZE.y - self.pos.y * metre_in_pixels.y,
            self.size.x * metre_in_pixels.x,
            self.size.y * metre_in_pixels.y,
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
                ui.horizontal(|ui| {
                    ui.label("Restitution:");
                    ui.add(egui::Slider::new(&mut self.restitution, (0.1)..=1.));
                });
                
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
    fn get_radius(&self) -> Option<f32> {
        None
    }
    fn set_vel(&mut self, new_vel: Vec2) {
        self.vel = new_vel;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
    fn get_size(&self) -> Option<Vec2> {
        Some(self.size)
    }
    fn get_restitution(&self) -> Option<f32> {
        Some(self.restitution)
    }
    fn as_trait(&self) -> &dyn RigidBody {
        self as &dyn RigidBody
    }
}
