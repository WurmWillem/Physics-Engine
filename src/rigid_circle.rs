use egui_macroquad::egui::Context;
use macroquad::prelude::*;

use crate::{
    engine::{RigidBodies, RigidBody},
    scenes::Variables,
    SCREEN_SIZE,
};

pub const WORLD_SIZE: Vec2 = vec2(60., 52.5);
pub const METRE_IN_PIXELS: Vec2 = vec2(SCREEN_SIZE.x / WORLD_SIZE.x, SCREEN_SIZE.y / WORLD_SIZE.y);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RigidCirle {
    enabled: bool,
    mass: f32,
    radius: f32,
    pos: Vec2,
    vel: Vec2,
    //f_res: Vec2,
    //f_g: Vec2,
    //default_pos: Vec2,
    //default_mass: f32,
}
impl RigidCirle {
    pub fn new(mass: f32, pos: Vec2, radius: f32) -> Self {
        Self {
            enabled: true,
            mass,
            radius,
            pos,
            vel: Vec2::ZERO,
            //f_res: Vec2::ZERO,
            //f_g: Vec2::ZERO,
            //default_pos: pos,
            //default_mass: mass,
        }
    }
}
impl RigidBody for RigidCirle {
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
            //f_res = Vec2::ZERO;
        } else if next_pos.y - self.radius - 1. < 0. {
            self.vel.y *= -1.;
            self.pos.y = self.radius + 1.;
        } else {
            self.pos = next_pos;
        }
    }
    fn draw(&self) {
        draw_circle(
            self.pos.x * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRE_IN_PIXELS.y,
            self.radius * METRE_IN_PIXELS.x,
            RED,
        )
    }
    fn update_ui(&mut self, _egui_ctx: &Context, _index: usize) {}
    fn get_enabled(&self) -> bool {
        self.enabled
    }
}
