use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{engine::Variables, rigid_body::RigidBodyType};

use crate::{
    rigid_body::{Forces, RigidBody},
    SCREEN_SIZE,
};

#[derive(Debug, Clone, Copy)]
pub struct RigidCircle {
    enabled: bool,
    mass: f32,
    radius: f32,
    restitution: f32,
    pos: Vec2,
    vel: Vec2,
    forces: Forces,
    default_pos: Vec2,
    default_mass: f32,
}
impl RigidCircle {
    pub fn new(mass: f32, pos: Vec2, radius: f32) -> Self {
        let forces = Forces::new(true, true, false);
        Self {
            enabled: true,
            mass,
            radius,
            restitution: 1.,
            pos,
            vel: vec2(10., 0.),
            forces,
            default_pos: pos,
            default_mass: mass,
        }
    }
}
impl RigidBody for RigidCircle {
    fn apply_forces(&mut self, vars: Variables, delta_time: f32, scene_size: Vec2) {
        let mut f_res = Vec2::ZERO;
        let mut f_g = None;
        let mut f_air = None;

        if let Some(g) = vars.g {
            // Fz = m * g
            let f_gravity = g * self.mass;
            f_res.y -= f_gravity;
            f_g = Some(f_gravity);
        }

        if let Some(c) = vars.c {
            // F_Air = 0.5 * p * A * v*v = c * A * v*v in our case because k = 0.5 * p
            let f_air_resistance = c * self.radius * 2. * self.vel * self.vel.abs();
            f_res -= f_air_resistance;
            f_air = Some(f_air_resistance)
        }

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_time;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_time;

        if next_pos.y + self.radius > scene_size.y {
            self.vel.y *= -self.restitution;
            self.pos.y = scene_size.y - self.radius;
        } else if next_pos.y - self.radius - 1. < 0. {
            self.vel.y *= -self.restitution;
            self.pos.y = self.radius + 1.;
        } else if next_pos.x + self.radius > scene_size.x {
            self.vel.x *= -self.restitution;
            self.pos.x = scene_size.x - self.radius;
        } else if next_pos.x - self.radius < 0. {
            self.vel.x *= -self.restitution;
            self.pos.x = self.radius;
        } else {
            self.pos = next_pos;
        }
        self.forces.f_res = f_res;
        self.forces.f_g = f_g;
        self.forces.f_air = f_air;
    }

    fn draw(&self, metre_in_pixels: Vec2) {
        draw_circle(
            self.pos.x * metre_in_pixels.x,
            SCREEN_SIZE.y - self.pos.y * metre_in_pixels.y,
            self.radius * metre_in_pixels.x,
            RED,
        )
    }

    fn update_based_on_ui(&mut self, egui_ctx: &Context, index: usize) {
        egui::Window::new(format!("Bouncing ball {index}")).show(egui_ctx, |ui| {
            ui.set_max_width(200.);

            ui.collapsing("Show", |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.enabled, "enabled");
                    if ui.button("Reset all").clicked() {
                        *self = RigidCircle::new(self.default_mass, self.default_pos, self.radius);
                    }
                });

                ui.collapsing("Show data", |ui| {
                    ui.heading("Data");
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
        });
    }
    fn get_type(&self) -> RigidBodyType {
        RigidBodyType::Circle
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
        Some(self.radius)
    }
    fn set_vel(&mut self, new_vel: Vec2) {
        self.vel = new_vel;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
    fn get_size(&self) -> Option<Vec2> {
        None
    }
    fn get_restitution(&self) -> Option<f32> {
        Some(self.restitution)
    }
    fn as_trait(&self) -> &dyn RigidBody {
        self as &dyn RigidBody
    }
}
