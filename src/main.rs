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
    g: i32,
    k: f32,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            rb: RigidBody::new(100.),
            g: 0,
            k: 0.,
        }
    }
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::R) {
            self.rb = RigidBody::new(100.);
        }

        self.update_ui();

        self.rb.apply_forces(self.g, self.k);
    }
    pub fn draw(&self) {
        self.rb.draw();
    }

    fn update_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.label(format!("FPS: {}", get_fps()));
                ui.separator();

                ui.heading("Rigidbody");
                ui.label(format!("Mass: {}", self.rb.mass));
                ui.horizontal(|ui| {
                    ui.label(format!("Velocity: {}", self.rb.vel));
                    if ui.button("Reset").clicked() {
                        self.rb.vel = Vec2::new(0., 0.);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!("Position: {}", self.rb.pos));
                    if ui.button("Reset").clicked() {
                        self.rb.pos = Vec2::new(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5);
                    }
                });

                ui.separator();

                ui.heading("Forces");
                ui.label("Gravity: ");
                ui.horizontal(|ui| {
                    ui.label("g: ");
                    ui.add(egui::Slider::new(&mut self.g, -300..=300));
                    if ui.button("Reset").clicked() {
                        self.g = 0;
                    }
                });
                ui.label("Air resistance: ");
                ui.horizontal(|ui| {
                    ui.label("k: ");
                    ui.add(egui::Slider::new(&mut self.k, (-1.)..=10.));
                    if ui.button("Reset").clicked() {
                        self.k = 0.;
                    }
                });
            });
        });
    }
}

#[allow(dead_code)]
fn pr<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
