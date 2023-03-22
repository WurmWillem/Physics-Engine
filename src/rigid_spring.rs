use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{
    engine::Variables,
    rigid_body::{Forces, RigidBody, RigidBodyType},
    SCREEN_SIZE,
};

pub struct RigidSpring {
    enabled: bool,
    mass: f32,
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    equilibrium: f32,
    c: f32,
    u: f32,
    clicked: bool,
    forces: Forces,
    default_mass: f32,
    default_pos: Vec2,
    default_size: Vec2,
}
impl RigidSpring {
    pub fn new(mass: f32, pos: Vec2, size: Vec2) -> Self {
        Self {
            enabled: true,
            mass,
            pos,
            vel: Vec2::ZERO,
            size,
            equilibrium: pos.y,
            c: 5.,
            u: 0.,
            clicked: false,
            forces: Forces::new(false, false, true),
            default_mass: mass,
            default_pos: pos,
            default_size: size,
        }
    }
}
impl RigidBody for RigidSpring {
    fn apply_forces(&mut self, _vars: Variables, delta_time: f32, scene_size: Vec2) {
        if is_mouse_button_down(MouseButton::Left) && !self.clicked {
            let mut mouse_pos = (mouse_position_local() + 1.) * 0.5 * scene_size;
            mouse_pos.y = scene_size.y - mouse_pos.y;
            if self.contains(mouse_pos) {
                self.clicked = true;
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.clicked = false;
        }
        if self.clicked {
            let mouse_y = scene_size.y - (mouse_position_local().y + 1.) * 0.5 * scene_size.y;
            if mouse_y > 5. && mouse_y < self.equilibrium * 2. - 5. {
                self.pos.y = mouse_y;
            }
        }
        self.u = self.equilibrium - self.pos.y;

        let mut f_res = Vec2::ZERO;
        let mut f_spring = 0.;

        if !self.clicked {
            //F_spring = c * u
            f_spring = self.c * self.u;
            f_res.y += f_spring;
        }

        //a = f / m
        let acc = f_res / self.mass;

        //v = u + a * dt
        self.vel += acc * delta_time;

        //p = p + v * dt
        let next_pos = self.pos + self.vel * delta_time;

        if next_pos.y > 5. && next_pos.y < self.equilibrium * 2. - 5. {
            self.pos = next_pos;
        } else {
            self.vel.y = 0.;
        }
        self.forces.f_res = f_res;
        self.forces.f_spring = Some(f_spring);
    }

    fn draw(&self, metre_in_pixels: Vec2) {
        draw_rectangle(
            self.pos.x * metre_in_pixels.x,
            SCREEN_SIZE.y - self.pos.y * metre_in_pixels.y,
            self.size.x * metre_in_pixels.x,
            self.size.y * metre_in_pixels.y,
            BLACK,
        );
        draw_line(
            (self.pos.x + self.size.x * 0.5) * metre_in_pixels.x,
            SCREEN_SIZE.y - self.pos.y * metre_in_pixels.y,
            (self.pos.x + self.size.x * 0.5) * metre_in_pixels.x,
            SCREEN_SIZE.y - metre_in_pixels.y,
            self.size.x * 0.6,
            BLACK,
        );
        draw_line(
            (self.pos.x + self.size.x * 0.2) * metre_in_pixels.x,
            SCREEN_SIZE.y - metre_in_pixels.y * 1.5,
            (self.pos.x + self.size.x * 0.8) * metre_in_pixels.x,
            SCREEN_SIZE.y - metre_in_pixels.y * 1.5,
            metre_in_pixels.y,
            BLACK,
        );
    }

    fn update_based_on_ui(&mut self, egui_ctx: &egui_macroquad::egui::Context, index: usize) {
        egui::Window::new(format!("Spring {index}")).show(egui_ctx, |ui| {
            ui.set_max_width(200.);
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.enabled, "enabled");
                if ui.button("Reset all").clicked() {
                    *self =
                        RigidSpring::new(self.default_mass, self.default_pos, self.default_size);
                }
            });

            ui.collapsing("Show data", |ui| {
                ui.heading("Data");
                ui.label(format!("Size: {} m", self.size));

                let mut mass_copy = self.mass;
                self.update_default_properties_ui(ui, &mut mass_copy, self.default_pos);
                self.mass = mass_copy;

                ui.horizontal(|ui| {
                    ui.label("c:");
                    ui.add(egui::Slider::new(&mut self.c, (1.)..=30.));
                    ui.label("N/m");
                });
            });

            self.forces.display_ui(ui);
        });
    }

    fn get_type(&self) -> RigidBodyType {
        RigidBodyType::Spring
    }
    fn get_enabled(&self) -> bool {
        self.enabled
    }
    fn get_pos(&self) -> Vec2 {
        self.pos
    }
    fn get_vel(&self) -> Vec2 {
        self.vel
    }
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn get_radius(&self) -> Option<f32> {
        None
    }
    fn get_restitution(&self) -> Option<f32> {
        None
    }
    fn set_vel(&mut self, new_vel: Vec2) {
        self.vel = new_vel;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
    fn get_size(&self) -> Option<Vec2> {
        Some(self.size)
    }
    fn as_trait(&self) -> &dyn RigidBody {
        self as &dyn RigidBody
    }
}
impl RigidSpring {
    fn contains(&self, point: Vec2) -> bool {
        point.x > self.pos.x
            && point.x < self.pos.x + self.size.x
            && point.y < self.pos.y
            && point.y > self.pos.y - self.size.y
    }
}
