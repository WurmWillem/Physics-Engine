use egui_macroquad::egui::{self, Context};
use macroquad::prelude::*;

use crate::{
    rigid_circle::{self, RigidCirle},
    rigid_square::{self, RigidSquare},
    SCREEN_SIZE,
};

pub struct Engine {
    scene: Scene,
    rigid_bodies: Vec<Box<dyn RigidBody>>,
    time_mult: f32,
    pause: bool,
    world_size: Vec2,
    g: f32,
    c: f32,
}
impl Engine {
    pub fn new(scene: Scene) -> Self {
        let rigid_bodies = scene.get_rigid_bodies();
        Self {
            scene,
            rigid_bodies,
            time_mult: 1.,
            pause: false,
            world_size: vec2(60., 52.5),
            g: 0.,
            c: 1.,
        }
    }
    pub fn update(&mut self) {
        self.update_ui();
        if !self.pause {
            self.rigid_bodies.iter_mut().for_each(|rb| {
                if rb.get_enabled() {
                    rb.apply_forces(self.g, self.c, self.time_mult);
                }
            });
        }
    }
    pub fn draw(&self) {
        draw_background(self.scene);
        self.rigid_bodies.iter().for_each(|rb| {
            if rb.get_enabled() {
                rb.draw();
            }
        });
    }

    fn update_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.set_max_width(190.);

                ui.heading("General");
                if ui.button("Next scene").clicked() {
                    *self = Engine::new(self.scene.get_next_scene());
                }
                ui.label(format!("FPS: {}", get_fps()));
                ui.horizontal(|ui| {
                    ui.label(format!("Time multiplier: "))
                        .on_hover_text("delta time gets multiplied by this");
                    ui.add(egui::Slider::new(&mut self.time_mult, (0.)..=2.));
                });
                if ui.button("Reset to 1").clicked() {
                    self.time_mult = 1.;
                }
                ui.label(format!("World size: {} m", self.world_size));
                ui.horizontal(|ui| {
                    if ui.button("Reset all").clicked() {
                        *self = Engine::new(self.scene);
                    }
                    ui.checkbox(&mut self.pause, "pause");
                });

                if self.scene == Scene::FallingSquares {
                    ui.separator();
                    ui.heading("Variables").on_hover_text(
                        "Variables used in equations to deduce the forces applied to each rigidbody",
                    );

                    ui.horizontal(|ui| {
                        ui.label("g:").on_hover_text("Acceleration due to gravity");
                        ui.add(egui::Slider::new(&mut self.g, (-30.)..=30.));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Reset to default").clicked() {
                            self.g = 9.81;
                        }
                        if ui.button("Reset to 0").clicked() {
                            self.g = 0.;
                        }
                    });
                    ui.separator();
    
                    ui.horizontal(|ui| {
                        ui.label("c:")
                            .on_hover_text("Multiplier for the air resistance");
                        ui.add(egui::Slider::new(&mut self.c, (-1.)..=30.));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Reset to default").clicked() {
                            self.c = 1.;
                        }
                        if ui.button("Reset to 0").clicked() {
                            self.c = 0.;
                        }
                    });
                }
                
            });

            for i in 0..self.rigid_bodies.len() {
                self.rigid_bodies[i].update_ui(egui_ctx, i + 1);
            }
        });
    }
}

pub trait RigidBody {
    fn apply_forces(&mut self, g: f32, c: f32, time_mult: f32);
    fn draw(&self);
    fn update_ui(&mut self, egui_ctx: &Context, index: usize);
    fn get_enabled(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    FallingSquares,
    SolarSystem,
}
impl Scene {
    fn get_world_size(&self) -> Vec2 {
        match self {
            Scene::FallingSquares => rigid_square::WORLD_SIZE,
            Scene::SolarSystem => rigid_circle::WORLD_SIZE,
        }
    }
    fn get_rigid_bodies(&self) -> Vec<Box<dyn RigidBody>> {
        let world_size = self.get_world_size();
        match self {
            Scene::FallingSquares => {
                let pos0 = vec2(world_size.x * 0.45, world_size.y * 0.5);
                let size0 = vec2(2., 2.);
                let pos1 = vec2(world_size.x * 0.55, world_size.y * 0.5);
                let size1 = vec2(2., 2.);

                let rs0 = RigidSquare::new(10., pos0, size0);
                let rs1 = RigidSquare::new(100., pos1, size1);
                vec![Box::new(rs0), Box::new(rs1)]
            }
            Scene::SolarSystem => {
                let pos0 = vec2(world_size.x * 0.5, world_size.y * 0.5);
                let rc0 = RigidCirle::new(10., pos0, 5.);
                vec![Box::new(rc0)]
            }
        }
    }
    fn get_next_scene(&self) -> Self {
        match self {
            Scene::FallingSquares => Scene::SolarSystem,
            Scene::SolarSystem => Scene::FallingSquares,
        }
    }
}

fn draw_background(scene: Scene) {
    if scene == Scene::FallingSquares {
        let world_size = scene.get_world_size();
        let metre_in_pixels = SCREEN_SIZE / world_size;
        for x in 0..=(world_size.x as usize) {
            draw_line(
                x as f32 * metre_in_pixels.x,
                0.,
                x as f32 * metre_in_pixels.x,
                SCREEN_SIZE.y,
                0.5,
                BLACK,
            )
        }
        for y in 0..=(world_size.y as usize) {
            draw_line(
                0.,
                SCREEN_SIZE.y - y as f32 * metre_in_pixels.y,
                SCREEN_SIZE.x,
                SCREEN_SIZE.y - y as f32 * metre_in_pixels.y,
                0.5,
                BLACK,
            )
        }
        draw_line(
            0.,
            SCREEN_SIZE.y,
            SCREEN_SIZE.x,
            SCREEN_SIZE.y,
            metre_in_pixels.y * 2.,
            BROWN,
        )
    }
}
