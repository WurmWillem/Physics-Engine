use macroquad::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const GRAVITY: f32 = 9.81 * 30.;
pub struct RigidBody {
    mass: f32,
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
}
impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            pos: Vec2::new(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.8),
            vel: Vec2::new(0., 0.),
            size: Vec2::new(80., 80.),
        }
    }
    pub fn apply_forces(&mut self) {
        let delta_t = get_frame_time();

        let mut f_res = Vec2::new(0., 0.);

        //Fz = m * a
        f_res.y -= GRAVITY * self.mass;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_t;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_t;

        //s = (u + a * dt)dt = a * dtdt + u * dt

        if next_pos.y - self.size.y > 0. {
            self.pos = next_pos;
        } else {
            self.vel.y = 0.;
            self.pos.y = self.size.y;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            SCREEN_HEIGHT - self.pos.y,
            self.size.x,
            self.size.y,
            RED,
        );
    }
}
