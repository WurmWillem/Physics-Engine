use std::ops::RangeInclusive;

use macroquad::prelude::*;

use crate::{
    bouncing_ball::{self, BouncingBall},
    engine::Variables,
    rigid_body::RigidBody,
    rigid_square::{self, RigidSquare},
    SCREEN_SIZE,
};

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
                let radius_0 = 0.5 * 5.;
                let radius_1 = 1. * 5.;
                let pos0 = vec2(world_size.x * 0.4, world_size.y * 0.8 + radius_0);
                let pos1 = vec2(world_size.x * 0.6, world_size.y * 0.8 + radius_1);
                //let pos0 = vec2(world_size.x * 0.4, radius_0 + 1.);
                //let pos1 = vec2(world_size.x * 0.6, radius_1 + 1.);

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
                g: Some(9.81),
                c: Some(0.),
            },
        }
    }
    pub fn get_c_range(&self) -> RangeInclusive<f32> {
        match self {
            Scene::FallingSquares => (-1.)..=30.,
            Scene::BouncingBall => (-0.01)..=1.,
        }
    }
    pub fn get_c_default(&self) -> f32 {
        match self {
            Scene::FallingSquares => 1.,
            Scene::BouncingBall => 0.01,
        }
    }
    pub fn draw_background(&self) {
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
