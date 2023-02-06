//use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{pr, METRES_TO_PIXELS, SCREEN_SIZE, SCREEN_SIZE_METRES};

pub struct RigidBody {
    pub mass: f32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
}
impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            pos: Vec2::new(SCREEN_SIZE_METRES.x * 0.5, SCREEN_SIZE_METRES.y * 0.5),
            vel: Vec2::ZERO,
            size: Vec2::new(2., 2.),
        }
    }
    pub fn apply_forces(&mut self, g: i32, k: f32) {
        let delta_t = get_frame_time();

        let mut f_res = Vec2::ZERO;

        //F_Air = 0.5 * p * A * v*v = k * v*v in our case because k = 0.5 * p * A
        f_res -= k * self.vel * self.vel.abs();

        //Fz = m * g
        f_res.y -= g as f32 * self.mass;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_t;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_t;

        if next_pos.y > SCREEN_SIZE_METRES.y {
            self.vel.y = 0.;
            self.pos.y = SCREEN_SIZE_METRES.y;
        } else if next_pos.y - self.size.y < 0. {
            self.vel.y = 0.;
            self.pos.y = self.size.y;
        } else {
            self.pos = next_pos;
        }
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
