use macroquad::prelude::*;

mod engine;
mod rigid_body;

use engine::Engine;

pub const SCREEN_SIZE: Vec2 = Vec2::new(800., 700.);
pub const SCREEN_SIZE_METRES: Vec2 = Vec2::new(20., 17.5);
pub const METRE_IN_PIXELS: Vec2 = Vec2::new(
    SCREEN_SIZE.x / SCREEN_SIZE_METRES.x,
    SCREEN_SIZE.y / SCREEN_SIZE_METRES.y,
);
//const C: Color = Color::new(0.2, 0.2, 0.2, 0.2);

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
