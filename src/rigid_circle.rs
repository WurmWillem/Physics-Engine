//use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

pub struct RigidCirle {
    pub enabled: bool,
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