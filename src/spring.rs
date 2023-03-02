use macroquad::prelude::*;

use crate::{
    engine::Variables,
    pr,
    rigid_body::{RigidBody, RigidBodyType},
    SCREEN_SIZE,
};

pub const WORLD_SIZE: Vec2 = vec2(60., 52.5);
pub const METRE_IN_PIXELS: Vec2 = vec2(SCREEN_SIZE.x / WORLD_SIZE.x, SCREEN_SIZE.y / WORLD_SIZE.y);

pub struct Spring {
    enabled: bool,
    mass: f32,
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    equilibrium: f32,
    c: f32,
    u: f32,
    clicked: bool,
}
impl Spring {
    pub fn new(mass: f32, pos: Vec2) -> Self {
        Self {
            enabled: true,
            mass,
            pos,
            vel: Vec2::ZERO,
            size: vec2(30., 3.),
            equilibrium: pos.y,
            c: 10.,
            u: 0.,
            clicked: false,
        }
    }
}
impl RigidBody for Spring {
    fn apply_forces(&mut self, _vars: Variables, delta_time: f32) {
        if is_mouse_button_down(MouseButton::Left) && !self.clicked {
            let mut mouse_pos = (mouse_position_local() + 1.) * 0.5 * WORLD_SIZE;
            mouse_pos.y = WORLD_SIZE.y - mouse_pos.y;
            if self.contains(mouse_pos) {
                self.clicked = true;
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.clicked = false;
        }
        if self.clicked {
            let mouse_y = WORLD_SIZE.y - (mouse_position_local().y + 1.) * 0.5 * WORLD_SIZE.y;
            self.pos.y = mouse_y;
        }
        self.u = self.equilibrium - self.pos.y;

        let mut f_res = Vec2::ZERO;

        //F_spring = c * u
        f_res.y += self.c * self.u;

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_time;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_time;

        //if next_pos.y - self.size.y > 0. && next_pos.y < WORLD_SIZE.y {
        self.pos = next_pos;
        //}
    }
    fn draw(&self) {
        draw_rectangle(
            self.pos.x * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRE_IN_PIXELS.y,
            self.size.x * METRE_IN_PIXELS.x,
            self.size.y * METRE_IN_PIXELS.y,
            BLACK,
        );
        draw_line(
            (self.pos.x + self.size.x * 0.5) * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - self.pos.y * METRE_IN_PIXELS.y,
            (self.pos.x + self.size.x * 0.5) * METRE_IN_PIXELS.x,
            SCREEN_SIZE.y - METRE_IN_PIXELS.y,
            self.size.x * 0.6,
            BLACK,
        );
    }
    fn update_based_on_ui(&mut self, _egui_ctx: &egui_macroquad::egui::Context, _index: usize) {}

    fn get_type(&self) -> RigidBodyType {
        RigidBodyType::Spring
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
        panic!("Spring doesnt have radius field")
    }
    fn set_vel(&mut self, new_vel: Vec2) {
        self.vel = new_vel;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
}
impl Spring {
    fn contains(&self, point: Vec2) -> bool {
        //pr(self.pos.x);
        //pr(point);
        if point.x > self.pos.x {
            //pr("yeh")
        }
        point.x > self.pos.x
            && point.x < self.pos.x + self.size.x
            && point.y < self.pos.y
            && point.y > self.pos.y - self.size.y
    }
}
