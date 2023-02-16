use macroquad::prelude::*;

mod engine;
mod rigid_circle;
mod rigid_square;

use engine::{Engine, Scene};

pub const SCREEN_SIZE: Vec2 = vec2(800., 700.);

#[macroquad::main("Physics Engine")]
async fn main() {
    request_new_screen_size(SCREEN_SIZE.x, SCREEN_SIZE.y);

    let mut engine = Engine::new(Scene::FallingSquares);

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
