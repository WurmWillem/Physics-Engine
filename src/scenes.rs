use egui_macroquad::egui::Ui;
use macroquad::prelude::*;

use crate::{
    bouncing_ball::{self, BouncingBall},
    engine::{RigidBody, Variables},
    rigid_square::{self, RigidSquare},
    SCREEN_SIZE,
};

const DIGITS_AFTER_DECIMAL: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    FallingSquares,
    BouncingBall,
}
impl Scene {
    pub fn get_world_size(&self) -> Vec2 {
        match self {
            Scene::FallingSquares => rigid_square::WORLD_SIZE,
            Scene::BouncingBall => bouncing_ball::WORLD_SIZE,
        }
    }
    pub fn get_rigid_bodies(&self) -> Vec<Box<dyn RigidBody>> {
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
            Scene::BouncingBall => {
                let radius_0 = 0.5;
                let radius_1 = 1.;
                let pos0 = vec2(world_size.x * 0.4, world_size.y * 0.5 + radius_0);
                let pos1 = vec2(world_size.x * 0.6, world_size.y * 0.5 + radius_1);

                let rc0 = BouncingBall::new(1., pos0, radius_0);
                let rc1 = BouncingBall::new(10., pos1, radius_1);
                vec![Box::new(rc0), Box::new(rc1)]
            }
        }
    }
    pub fn get_next_scene(&self) -> Self {
        match self {
            Scene::FallingSquares => Scene::BouncingBall,
            Scene::BouncingBall => Scene::FallingSquares,
        }
    }
    pub fn get_variables(&self) -> Variables {
        match self {
            Scene::FallingSquares => Variables {
                g: Some(0.),
                c: Some(1.),
            },
            Scene::BouncingBall => Variables {
                g: Some(0.),
                c: Some(0.1),
            },
        }
    }
    pub fn draw_background(&self) {
        if *self == Scene::FallingSquares || *self == Scene::BouncingBall {
            let world_size = self.get_world_size();
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
        let f = f.round();
        f / (DIGITS_AFTER_DECIMAL + 1) as f32
    }
}
impl Format for Vec2 {
    fn format(&self) -> Self {
        vec2(self.x.format(), self.y.format())
    }
}
