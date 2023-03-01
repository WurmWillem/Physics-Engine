use egui_macroquad::egui::{Context, Ui, self};
use macroquad::prelude::{vec2, Vec2};

use crate::engine::Variables;

pub type RigidBodies = Vec<Box<dyn RigidBody>>;

const DIGITS_AFTER_DECIMAL: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidBodyType {
    RigidSquare,
    RigidBall,
}

pub trait RigidBody {
    fn apply_forces(&mut self, vars: Variables, delta_time: f32);
    fn draw(&self);
    fn update_based_on_ui(&mut self, egui_ctx: &Context, index: usize);
    fn get_type(&self) -> RigidBodyType;
    fn get_enabled(&self) -> bool;
    fn get_pos(&self) -> Vec2;
    fn get_vel(&self) -> Vec2;
    fn get_mass(&self) -> f32;
    fn get_radius(&self) -> f32;
    fn set_vel(&mut self, new_vel: Vec2);
    fn set_pos(&mut self, new_pos: Vec2);

    fn update_default_properties_ui(&mut self, ui: &mut Ui, mass: &mut f32, default_pos: Vec2) {
        ui.horizontal(|ui| {
            ui.label("Mass:");
            ui.add(egui::Slider::new(mass, (0.1)..=300.));
            ui.label("kg");
        });
        ui.horizontal(|ui| {
            ui.label(format!("Velocity: {} m/s", self.get_vel().format()));
            if ui.button("Reset").clicked() {
                self.set_vel(Vec2::ZERO);
            }
        });
        ui.horizontal(|ui| {
            ui.label(format!("Position: {} m", self.get_pos().format()));
            if ui.button("Reset").clicked() {
                self.set_pos(default_pos);
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Forces {
    pub f_res: Vec2,
    pub f_g: Option<f32>,
    pub f_air: Option<Vec2>,
}
impl Forces {
    pub fn new(f_g_used: bool, f_air_used: bool) -> Self {
        let f_g = if f_g_used { Some(0.) } else { None };
        let f_air = if f_air_used { Some(Vec2::ZERO) } else { None };
        Self {
            f_res: Vec2::ZERO,
            f_g,
            f_air,
        }
    }
    pub fn display_ui(&self, ui: &mut Ui) {
        ui.collapsing("Show forces", |ui| {
            ui.label(format!(
                "F_res = {} = {} N",
                self.f_res.format(),
                self.f_res.length().format()
            ));
            if let Some(f_g) = self.f_g {
                ui.label(format!("Gravity: m * g = {} N", f_g.format()));
            }
            if let Some(f_air) = self.f_air {
                ui.label("Air resistance: c * A * v*v =");
                ui.label(format!(
                    "{} = {} N",
                    f_air.format(),
                    f_air.length().format()
                ));
            }
        });
    }
}

pub trait Format {
    fn format(&self) -> Self;
}
impl Format for f32 {
    fn format(&self) -> Self {
        let f = *self * (DIGITS_AFTER_DECIMAL + 1) as f32;
        f.round() / (DIGITS_AFTER_DECIMAL + 1) as f32
    }
}
impl Format for Vec2 {
    fn format(&self) -> Self {
        vec2(self.x.format(), self.y.format())
    }
}
