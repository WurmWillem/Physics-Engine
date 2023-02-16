use macroquad::prelude::*;

mod engine;
mod rigid_square;
mod rigid_circle;

use engine::Engine;

pub const SCREEN_SIZE: Vec2 = vec2(800., 700.);
pub const SCREEN_SIZE_METRES: Vec2 = vec2(60., 52.5);
pub const METRE_IN_PIXELS: Vec2 = vec2(
    SCREEN_SIZE.x / SCREEN_SIZE_METRES.x,
    SCREEN_SIZE.y / SCREEN_SIZE_METRES.y,
);

#[macroquad::main("Physics Engine")]
async fn main() {
    request_new_screen_size(SCREEN_SIZE.x, SCREEN_SIZE.y);

    let mut engine = Engine::new();
   
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
