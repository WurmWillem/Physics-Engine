use egui_macroquad::egui::{self, Context, Ui};
use macroquad::prelude::{vec2, Vec2};

use crate::engine::Variables;

pub type RigidBodies = Vec<Box<dyn RigidBody>>;
const DIGITS_AFTER_DECIMAL: usize = 0;

pub trait RigidBody {
    fn apply_forces(&mut self, vars: Variables, delta_time: f32, scene_size: Vec2);
    fn draw(&self, metre_in_pixels: Vec2);
    fn update_based_on_ui(&mut self, egui_ctx: &Context, index: usize);
    fn get_type(&self) -> RigidBodyType;
    fn get_enabled(&self) -> bool;
    fn get_pos(&self) -> Vec2;
    fn get_vel(&self) -> Vec2;
    fn get_mass(&self) -> f32;
    fn get_radius(&self) -> Option<f32>;
    fn get_size(&self) -> Option<Vec2>;
    fn set_vel(&mut self, new_vel: Vec2);
    fn set_pos(&mut self, new_pos: Vec2);
    fn as_trait(&self) -> &dyn RigidBody;

    fn colliding(&self, rb1: &Box<dyn RigidBody>) -> bool {
        if self.get_type() == RigidBodyType::Circle && rb1.get_type() == RigidBodyType::Circle {
            if let Some(radius_0) = self.get_radius() {
                if let Some(radius_1) = rb1.get_radius() {
                    let dist_between_circles = self.get_pos().distance(rb1.get_pos());
                    return dist_between_circles < radius_0 + radius_1;
                }
            }
        } else if (self.get_type() == RigidBodyType::Square
            && rb1.get_type() == RigidBodyType::Circle)
            || (self.get_type() == RigidBodyType::Circle && rb1.get_type() == RigidBodyType::Square)
        {
            let (size, radius) = get_size_and_radius(self.as_trait(), rb1);

            let rect_size_half = size * 0.5;
            let circle_dist = vec2(
                (rb1.get_pos().x - rect_size_half.x - self.get_pos().x).abs(),
                (rb1.get_pos().y + rect_size_half.y - self.get_pos().y).abs(),
            );

            // Circle is too far away from rect to be colliding
            if circle_dist.x > (rect_size_half.x + radius)
                || circle_dist.y > (rect_size_half.y + radius)
            {
                return false;
            }

            // Circle is definitely colliding
            if circle_dist.x <= rect_size_half.x || circle_dist.y <= rect_size_half.y {
                return true;
            }

            // Check for corner case
            let corner_dist_square = (circle_dist.x - rect_size_half.x).powi(2)
                + (circle_dist.y - rect_size_half.y).powi(2);

            return corner_dist_square <= radius.powi(2);
        }
        false
    }

    fn update_default_properties_ui(&mut self, ui: &mut Ui, mass: &mut f32, default_pos: Vec2) {
        ui.horizontal(|ui| {
            ui.label("Mass:");
            ui.add(egui::Slider::new(mass, (0.1)..=300.));
            ui.label("kg");
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Velocity: {} m/s",
                self.get_vel().format(DIGITS_AFTER_DECIMAL)
            ));
            if ui.button("Reset").clicked() {
                self.set_vel(Vec2::ZERO);
            }
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Position: {} m",
                self.get_pos().format(DIGITS_AFTER_DECIMAL)
            ));
            if ui.button("Reset").clicked() {
                self.set_pos(default_pos);
            }
        });
    }
}

fn get_size_and_radius(rb0: &dyn RigidBody, rb1: &Box<dyn RigidBody>) -> (Vec2, f32) {
    let mut new_size = Vec2::ZERO;
    let mut new_radius = 0.;
    if let Some(s) = rb0.get_size() {
        new_size = s;
        if let Some(r) = rb1.get_radius() {
            new_radius = r;
        }
    }
    if let Some(s) = rb1.get_size() {
        new_size = s;
        if let Some(r) = rb0.get_radius() {
            new_radius = r;
        }
    }
    if new_size == Vec2::ZERO || new_radius == 0. {
        panic!("Both properties are None")
    }
    (new_size, new_radius)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidBodyType {
    Square,
    Circle,
    Spring,
}

#[derive(Debug, Clone, Copy)]
pub struct Forces {
    pub f_res: Vec2,
    pub f_g: Option<f32>,
    pub f_air: Option<Vec2>,
    pub f_spring: Option<f32>,
}
impl Forces {
    pub fn new(f_g_used: bool, f_air_used: bool, f_spring_used: bool) -> Self {
        let f_g = if f_g_used { Some(0.) } else { None };
        let f_air = if f_air_used { Some(Vec2::ZERO) } else { None };
        let f_spring = if f_spring_used { Some(0.) } else { None };
        Self {
            f_res: Vec2::ZERO,
            f_g,
            f_air,
            f_spring,
        }
    }
    pub fn display_ui(&self, ui: &mut Ui) {
        ui.collapsing("Show forces", |ui| {
            ui.label(format!(
                "F_res = {} = {} N",
                self.f_res.format(DIGITS_AFTER_DECIMAL),
                self.f_res.length().format(DIGITS_AFTER_DECIMAL)
            ));
            if let Some(f_g) = self.f_g {
                ui.label(format!(
                    "Gravity: m * g = {} N",
                    f_g.format(DIGITS_AFTER_DECIMAL)
                ));
            }
            if let Some(f_air) = self.f_air {
                ui.label("Air resistance: c * A * v*v =");
                ui.label(format!(
                    "{} = {} N",
                    f_air.format(DIGITS_AFTER_DECIMAL),
                    f_air.length().format(DIGITS_AFTER_DECIMAL)
                ));
            }
            if let Some(f_spring) = self.f_spring {
                ui.label(format!(
                    "Spring force: c * u = {} N",
                    f_spring.format(DIGITS_AFTER_DECIMAL)
                ));
            }
        });
    }
}

pub trait Format {
    fn format(&self, digits_after_decimal: usize) -> Self;
}
impl Format for f32 {
    fn format(&self, digits_after_decimal: usize) -> Self {
        let f = *self * 10_usize.pow(digits_after_decimal as u32) as f32;
        f.round() / 10_usize.pow(digits_after_decimal as u32) as f32
    }
}
impl Format for Vec2 {
    fn format(&self, digits_after_decimal: usize) -> Self {
        vec2(
            self.x.format(digits_after_decimal),
            self.y.format(digits_after_decimal),
        )
    }
}
