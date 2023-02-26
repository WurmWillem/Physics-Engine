use dyn_clone::DynClone;
use egui_macroquad::egui::{Context, Ui};
use macroquad::prelude::{vec2, Vec2};

pub type RigidBodies = Vec<Box<dyn RigidBody>>;

const DIGITS_AFTER_DECIMAL: usize = 0;

pub trait RigidBody: DynClone {
    fn apply_forces(&mut self, vars: Variables, time_mult: f32, rigid_bodies: &RigidBodies);
    fn draw(&self);
    fn update_ui(&mut self, egui_ctx: &Context, index: usize);
    fn get_enabled(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Variables {
    pub g: Option<f32>,
    pub c: Option<f32>,
}

pub trait Format {
    fn format(&self) -> Self;
}
impl Format for f32 {
    fn format(&self) -> Self {
        let f = *self * (DIGITS_AFTER_DECIMAL + 1) as f32;
        let f = f.round();
        f / (DIGITS_AFTER_DECIMAL + 1) as f32
    }
}
impl Format for Vec2 {
    fn format(&self) -> Self {
        vec2(self.x.format(), self.y.format())
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
