use egui_macroquad::egui::Context;
//use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{
    engine::{RigidBody, Variables},
    SCREEN_SIZE,
}; //0.69634

pub const WORLD_SIZE: Vec2 = vec2(6.9634, 7.9582);
pub const MEGA_METRE_IN_PIXELS: Vec2 =
    vec2(SCREEN_SIZE.x / WORLD_SIZE.x, SCREEN_SIZE.y / WORLD_SIZE.y);

pub struct RigidCirle {
    enabled: bool,
    mass: f32,
    radius: f32,
    pos: Vec2,
    vel: Vec2,
    f_res: Vec2,
    f_g: Vec2,
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
            f_res: Vec2::ZERO,
            f_g: Vec2::ZERO,
            //default_pos: pos,
            //default_mass: mass,
        }
    }
}
impl RigidBody for RigidCirle {
    fn apply_forces(&mut self, _vars: Variables, _time_mult: f32) {
        //todo!()
    }
    fn draw(&self) {
        draw_circle(
            self.pos.x * MEGA_METRE_IN_PIXELS.x,
            self.pos.y * MEGA_METRE_IN_PIXELS.y,
            self.radius * MEGA_METRE_IN_PIXELS.x,
            RED,
        )
    }
    fn update_ui(&mut self, _egui_ctx: &Context, _index: usize) {}
    fn get_enabled(&self) -> bool {
        self.enabled
    }
}
