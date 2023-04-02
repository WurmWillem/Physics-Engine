use macroquad::prelude::*;

mod engine;
mod rigid_body;
mod rigid_circle;
mod rigid_rectangle;
mod rigid_spring;
mod scenes;

use engine::Engine;
use scenes::Scene;

pub const SCREEN_X_INCREASE: f32 = 1.7;
pub const SCREEN_SIZE: Vec2 = vec2(700. * SCREEN_X_INCREASE, 700.);

#[macroquad::main("Physics Engine")]
async fn main() {
    request_new_screen_size(SCREEN_SIZE.x, SCREEN_SIZE.y);

    let mut engine = Engine::new(Scene::FallingRectangles);

    loop {
        clear_background(LIGHTGRAY);

        engine.update();
        engine.draw();

        // Draw UI
        egui_macroquad::draw();

        next_frame().await
    }
}

#[allow(dead_code)]
fn pr<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
