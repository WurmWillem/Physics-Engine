use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;

use crate::{
    rigid_body::{Format, RigidBodies, RigidBodyType},
    scenes::Scene,
};

const TIME_INCREMENT: f32 = 0.1;

pub struct Engine {
    scene: Scene,
    rigid_bodies: RigidBodies,
    world_size: Vec2,
    vars: Variables,
    time_mult: f32,
    pause: bool,
    time_step_mode_enabled: bool,
    time_passed: f32,
}
impl Engine {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
            rigid_bodies: scene.get_rigid_bodies(),
            world_size: scene.get_world_size(),
            vars: scene.get_variables(),
            time_mult: 1.,
            pause: false,
            time_step_mode_enabled: false,
            time_passed: 0.,
        }
    }
    pub fn update(&mut self) {
        self.update_based_on_ui();

        //apply forces on the rigidbodies
        if !self.pause && !self.time_step_mode_enabled {
            let delta_time = self.time_mult * get_frame_time();
            self.rigid_bodies.iter_mut().for_each(|rb| {
                if rb.get_enabled() {
                    rb.apply_forces(self.vars, delta_time);
                }
            });
        }

        self.resolve_collisions();
    }
    pub fn draw(&self) {
        self.scene.draw_background();

        self.rigid_bodies.iter().for_each(|rb| {
            if rb.get_enabled() {
                rb.draw();
            }
        });
    }

    fn resolve_collisions(&mut self) {
        for j in 0..self.rigid_bodies.len() {
            for i in 0..self.rigid_bodies.len() {
                if j == i {
                    break;
                }
                if self.rigid_bodies[j].get_type() == RigidBodyType::Square
                    || self.rigid_bodies[i].get_type() == RigidBodyType::Square
                {
                    continue;
                }
                let rb0 = &self.rigid_bodies[j];
                let rb1 = &self.rigid_bodies[i];

                let distance_between_balls = rb0.get_pos().distance(rb1.get_pos());
                if distance_between_balls > rb0.get_radius() + rb1.get_radius() {
                    continue;
                }

                // Collision normal, the direction in which the impulse will be applied
                let normal = (rb1.get_pos() - rb0.get_pos()).normalize();

                // Calculate relative velocity
                let relative_vel = rb1.get_vel() - rb0.get_vel();

                // Calculate relative velocity in terms of the normal direction
                let vel_along_normal = normal.dot(relative_vel);
                if vel_along_normal > 0. {
                    continue;
                }

                // Coefficient of restitution, bounciness/elasticity. the higher the bouncier they will be
                let e = 1.;

                let inverse_mass_0 = 1. / rb0.get_mass();
                let inverse_mass_1 = 1. / rb1.get_mass();

                // Calculate impulse scalar
                let mut jay = -(1. + e) * vel_along_normal;
                jay /= inverse_mass_0 + inverse_mass_1;

                // Calculate new velocity based on impulse
                let impulse = jay * normal;
                let new_vel_0 = rb0.get_vel() - inverse_mass_0 * impulse;
                let new_vel_1 = rb1.get_vel() + inverse_mass_1 * impulse;

                // Set new velocities
                self.rigid_bodies[j].set_vel(new_vel_0);
                self.rigid_bodies[i].set_vel(new_vel_1);
                continue;
            }
        }
    }

    fn update_based_on_ui(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Physics Engine").show(egui_ctx, |ui| {
                ui.set_max_width(190.);

                ui.horizontal(|ui| {
                    ui.heading("General");
                    if ui.button("Next scene").clicked() {
                        *self = Engine::new(self.scene.get_next_scene());
                    }
                });

                ui.label(format!("FPS: {}", get_fps()));
                ui.label(format!("time passed: {}", self.time_passed.format(2)));
                ui.label(format!("World size: {} m", self.world_size));
                ui.horizontal(|ui| {
                    if ui.button("Reset scene").clicked() {
                        *self = Engine::new(self.scene);
                    }
                    if ui.button("Reset all entities").clicked() {
                        self.rigid_bodies = self.scene.get_rigid_bodies();
                    }
                });
                ui.separator();

                self.update_time(ui);
                self.vars.update_ui(ui, self.scene);
            });

            for i in 0..self.rigid_bodies.len() {
                self.rigid_bodies[i].update_based_on_ui(egui_ctx, i + 1);
            }
        });
    }

    fn update_time(&mut self, ui: &mut Ui) {
        ui.collapsing("Show time", |ui| {
            ui.checkbox(&mut self.time_step_mode_enabled, "time step mode enabled");
            if self.time_step_mode_enabled {
                ui.horizontal(|ui| {
                    ui.label("timestep:");
                    self.create_time_step_button(ui, "Next", TIME_INCREMENT);
                    if self.time_passed - TIME_INCREMENT >= 0. {
                        self.create_time_step_button(ui, "Previous", -TIME_INCREMENT);
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(format!("Time multiplier: "))
                        .on_hover_text("delta time gets multiplied by this");
                    ui.add(egui::Slider::new(&mut self.time_mult, (0.)..=10.));
                });
                ui.horizontal(|ui| {
                    if ui.button("Reset to 1").clicked() {
                        self.time_mult = 1.;
                    }
                    ui.checkbox(&mut self.pause, "pause");
                });
            }
        });
        if !self.time_step_mode_enabled {
            self.time_passed += get_frame_time();
        }
    }

    fn create_time_step_button(&mut self, ui: &mut Ui, title: &str, increment: f32) {
        if ui.button(title).clicked() {
            self.time_passed += increment;
            self.rigid_bodies.iter_mut().for_each(|rb| {
                if rb.get_enabled() {
                    rb.apply_forces(self.vars, increment);
                }
            });
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Variables {
    pub g: Option<f32>,
    pub c: Option<f32>,
}
impl Variables {
    pub fn new(g: Option<f32>, c: Option<f32>) -> Self {
        Variables { g, c }
    }
    pub fn update_ui(&mut self, ui: &mut Ui, scene: Scene) {
        ui.collapsing("Show variables", |ui| {
            if let Some(mut g) = self.g {
                ui.horizontal(|ui| {
                    ui.label("g:").on_hover_text("Acceleration due to gravity");
                    ui.add(egui::Slider::new(&mut g, (-30.)..=30.));
                });
                self.create_reset_buttons(ui, &mut g, 9.81);
                self.g = Some(g);
            }

            if let Some(mut c) = self.c {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("c:")
                        .on_hover_text("Multiplier for the air resistance");
                    ui.add(egui::Slider::new(&mut c, scene.get_c_range()));
                });
                self.create_reset_buttons(ui, &mut c, scene.get_c_default());
                self.c = Some(c);
            }
        });
    }

    fn create_reset_buttons(&self, ui: &mut Ui, var: &mut f32, default: f32) {
        ui.horizontal(|ui| {
            if ui.button("Reset to default").clicked() {
                *var = default;
            }
            if ui.button("Reset to 0").clicked() {
                *var = 0.;
            }
        });
    }
}
