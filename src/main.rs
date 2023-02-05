#![allow(non_snake_case)]
use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 700.;
const GRAVITY: f32 = 9.81 * 30.;

#[macroquad::main("Physics Engine")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut rb = RigidBody::new(100.);

    loop {
        clear_background(LIGHTGRAY);

        rb.apply_forces();
        rb.draw();

        next_frame().await
    }
}

struct RigidBody {
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

#[allow(dead_code)]
fn pr<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
