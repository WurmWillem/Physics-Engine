//use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{pr, METRES_TO_PIXELS, SCREEN_SIZE, SCREEN_SIZE_METRES};

pub struct RigidBody {
    pub mass: f32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub f_res: Vec2,
    pub f_g: f32,
    pub f_air: Vec2,
}
impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            pos: Vec2::new(SCREEN_SIZE_METRES.x * 0.5, SCREEN_SIZE_METRES.y * 0.5),
            vel: Vec2::ZERO,
            size: Vec2::new(2., 2.),
            f_res: Vec2::ZERO,
            f_g: 0.,
            f_air: Vec2::ZERO,
        }
    }
    pub fn apply_forces(&mut self, g: f32, k: f32) {
        let delta_t = get_frame_time();

        let mut f_res = Vec2::ZERO;

        //Fz = m * g
        let f_g = g * self.mass;
        f_res.y -= f_g;

        //F_Air = 0.5 * p * A * v*v = k * v*v in our case because k = 0.5 * p * A
        let f_air = k * self.vel * self.vel.abs();
        f_res -= f_air;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_t;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_t;

        if next_pos.y > SCREEN_SIZE_METRES.y {
            self.vel.y = 0.;
            self.pos.y = SCREEN_SIZE_METRES.y;
            f_res = Vec2::ZERO;
        } else if next_pos.y - self.size.y < 0. {
            self.vel.y = 0.;
            self.pos.y = self.size.y;
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
            self.pos.x * METRES_TO_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRES_TO_PIXELS.y,
            self.size.x * METRES_TO_PIXELS.x,
            self.size.y * METRES_TO_PIXELS.y,
            RED,
        );
    }
}
