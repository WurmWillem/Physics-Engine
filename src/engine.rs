use egui_macroquad::egui::{self};
use macroquad::prelude::*;

use crate::{
    rigid_body::{RigidBodies, Variables},
    scenes::Scene,
};

pub struct Engine {
    scene: Scene,
    rigid_bodies: RigidBodies,
    time_mult: f32,
    pause: bool,
    world_size: Vec2,
    vars: Variables,
}
impl Engine {
    pub fn new(scene: Scene) -> Self {
        let rigid_bodies = scene.get_rigid_bodies();
        let vars = scene.get_variables();
        Self {
            scene,
            rigid_bodies,
            time_mult: 1.,
            pause: false,
            world_size: scene.get_world_size(),
            vars,
        }
    }
    pub fn update(&mut self) {
        self.update_ui();
        if !self.pause {
            let rigid_bodies = clone_rigid_bodies(&self.rigid_bodies);
            self.rigid_bodies.iter_mut().for_each(|rb| {
                if rb.get_enabled() {
                    rb.apply_forces(self.vars, self.time_mult, &rigid_bodies);
                }
            });
        }
    }
    pub fn draw(&self) {
        self.scene.draw_background();
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
                    if ui.button("Reset everything").clicked() {
                        *self = Engine::new(self.scene);
                    }
                    if ui.button("Reset all entities").clicked() {
                        self.rigid_bodies = self.scene.get_rigid_bodies();
                    }
                    ui.checkbox(&mut self.pause, "pause");
                });

                ui.separator();
                ui.heading("Variables").on_hover_text(
                    "Variables used in equations to deduce the forces applied to each rigidbody",
                );

                if let Some(mut g) = self.vars.g {
                    ui.horizontal(|ui| {
                        ui.label("g:").on_hover_text("Acceleration due to gravity");
                        ui.add(egui::Slider::new(&mut g, (-30.)..=30.));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Reset to default").clicked() {
                            g = 9.81;
                        }
                        if ui.button("Reset to 0").clicked() {
                            g = 0.;
                        }
                    });
                    self.vars.g = Some(g);
                }

                if let Some(mut c) = self.vars.c {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("c:")
                            .on_hover_text("Multiplier for the air resistance");
                        ui.add(egui::Slider::new(&mut c, (-1.)..=30.));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Reset to default").clicked() {
                            c = 1.;
                        }
                        if ui.button("Reset to 0").clicked() {
                            c = 0.;
                        }
                    });
                    self.vars.c = Some(c);
                }
            });
            for i in 0..self.rigid_bodies.len() {
                self.rigid_bodies[i].update_ui(egui_ctx, i + 1);
            }
        });
    }
}

fn clone_rigid_bodies(v: &RigidBodies) -> RigidBodies {
    let mut rigid_bodies = Vec::new();
    for i in 0..v.len() {
        let rb = dyn_clone::clone_box(&*v[i]);
        rigid_bodies.push(rb);
    }
    rigid_bodies
}
