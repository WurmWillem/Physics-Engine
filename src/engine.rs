use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;

use crate::{
    rigid_body::{RigidBodies, RigidBodyType},
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
        Self {
            scene,
            rigid_bodies: scene.get_rigid_bodies(),
            time_mult: 1.,
            pause: false,
            world_size: scene.get_world_size(),
            vars: scene.get_variables(),
        }
    }
    pub fn update(&mut self) {
        self.update_based_on_ui();

        //apply forces on the rigidbodies
        if !self.pause {
            let rigid_bodies = clone_rigid_bodies(&self.rigid_bodies);
            self.rigid_bodies.iter_mut().for_each(|rb| {
                if rb.get_enabled() {
                    rb.apply_forces(self.vars, self.time_mult, &rigid_bodies);
                }
            });
        }

        //self.resolve_collisions();
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
                if j == i
                    || self.rigid_bodies[j].get_type() == RigidBodyType::RigidSquare
                    || self.rigid_bodies[i].get_type() == RigidBodyType::RigidSquare
                {
                    continue;
                }
                let rb0 = &self.rigid_bodies[j];
                let rb1 = &self.rigid_bodies[i];

                let distance_between_balls = rb0.get_pos().distance(rb1.get_pos());
                if distance_between_balls > rb0.get_radius() + rb1.get_radius() {
                    continue;
                }
                let force0 = 0.5 * rb0.get_mass() * rb0.get_vel().length_squared();
                let dist0 = (rb0.get_pos() - rb1.get_pos()).normalize();
                let force1 = 0.5 * rb1.get_mass() * rb1.get_vel().length_squared();
                let dist1 = (rb1.get_pos() - rb0.get_pos()).normalize();

                let vel0 = rb0.get_vel() + force0 * dist0 - force1 * dist1;
                let vel1 = rb1.get_vel() + force1 * dist1 - force0 * dist0;

                self.rigid_bodies[j].set_vel(vel0);
                self.rigid_bodies[i].set_vel(vel1);
                /*
                let distance_between_balls = self.pos.distance(rb.get_pos());
                if distance_between_balls > self.radius + rb.get_radius() {
                    continue;
                }
                let force = 0.5 * self.mass * self.vel.length_squared();
                let dist = (self.pos - rb.get_pos()).normalize();
                let force1 = 0.5 * rb.get_mass() * rb.get_vel().length_squared();
                let dist1 = (rb.get_pos() - self.pos).normalize();

                let vel = self.vel + force * dist - force1 * dist1;
                let new_pos = self.pos + vel * delta_t;

                if new_pos.distance(rb.get_pos()) < self.radius + rb.get_radius() {
                    //pr("off");
                    next_pos -= self.vel * delta_t
                } else {
                    self.vel += vel * delta_t * 0.1;
                }
                 */
            }
        }
    }

    fn update_based_on_ui(&mut self) {
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
                ui.horizontal(|ui| {
                    if ui.button("Reset to 1").clicked() {
                        self.time_mult = 1.;
                    }
                    ui.checkbox(&mut self.pause, "pause");
                });

                ui.label(format!("World size: {} m", self.world_size));
                ui.horizontal(|ui| {
                    if ui.button("Reset everything").clicked() {
                        *self = Engine::new(self.scene);
                    }
                    if ui.button("Reset all entities").clicked() {
                        self.rigid_bodies = self.scene.get_rigid_bodies();
                    }
                });

                ui.separator();
                ui.heading("Variables").on_hover_text(
                    "Variables used in equations to deduce the forces applied to each rigidbody",
                );
                self.vars.update_ui(ui, self.scene);
            });
            for i in 0..self.rigid_bodies.len() {
                self.rigid_bodies[i].update_based_on_ui(egui_ctx, i + 1);
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Variables {
    pub g: Option<f32>,
    pub c: Option<f32>,
}
impl Variables {
    pub fn update_ui(&mut self, ui: &mut Ui, scene: Scene) {
        if let Some(mut g) = self.g {
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
            self.g = Some(g);
        }

        if let Some(mut c) = self.c {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("c:")
                    .on_hover_text("Multiplier for the air resistance");
                ui.add(egui::Slider::new(&mut c, scene.get_c_range()));
            });
            ui.horizontal(|ui| {
                if ui.button("Reset to default").clicked() {
                    c = scene.get_c_default();
                }
                if ui.button("Reset to 0").clicked() {
                    c = 0.;
                }
            });
            self.c = Some(c);
        }
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
