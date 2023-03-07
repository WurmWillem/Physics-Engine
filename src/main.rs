use macroquad::prelude::*;

mod bouncing_ball;
mod engine;
mod rigid_body;
mod rigid_square;
mod scenes;
mod spring;

use engine::Engine;
use scenes::Scene;

pub const SCREEN_SIZE: Vec2 = vec2(800. * 1.5, 700.);

#[macroquad::main("Physics Engine")]
async fn main() {
    request_new_screen_size(SCREEN_SIZE.x, SCREEN_SIZE.y);

    let mut engine = Engine::new(Scene::SquareAndBall);

    loop {
        clear_background(LIGHTGRAY);

        engine.update();
        engine.draw();

        egui_macroquad::draw();

        next_frame().await
    }
}

#[allow(dead_code)]
fn pr<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
