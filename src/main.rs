use egui_macroquad::egui;
use macroquad::prelude::*;

mod rigid_body;
use rigid_body::RigidBody;

pub const SCREEN_WIDTH: f32 = 800.;
pub const SCREEN_HEIGHT: f32 = 700.;

#[macroquad::main("Physics Engine")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut engine = Engine::new();

    loop {
        clear_background(LIGHTGRAY);

        engine.update();
        engine.draw();
        
        egui_macroquad::draw();

        next_frame().await
    }
}

pub struct Engine {
    rb: RigidBody,
}
impl Engine {
    pub fn new() -> Self {
        Self { rb: RigidBody::new(100.) }
    }
    pub fn update(&mut self) {
        let mut gravity = 10;

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.heading("Forces");
                ui.horizontal(|ui| {
                    ui.label("Gravity: ");
                });
                ui.add(egui::Slider::new(&mut gravity, 0..=120).text("gravity"));
            });
        });
        self.rb.apply_forces();
    }
    pub fn draw(&self) {
        self.rb.draw();
    }
}

#[allow(dead_code)]
fn pr<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
