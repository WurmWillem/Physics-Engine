use macroquad::prelude::*;

use crate::{
    engine::RigidBody,
    rigid_circle::{self, RigidCirle},
    rigid_square::{self, RigidSquare},
    SCREEN_SIZE,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    FallingSquares,
    SolarSystem,
}
impl Scene {
    pub fn get_world_size(&self) -> Vec2 {
        match self {
            Scene::FallingSquares => rigid_square::WORLD_SIZE,
            Scene::SolarSystem => rigid_circle::WORLD_SIZE,
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
            Scene::SolarSystem => {
                let pos0 = vec2(world_size.x * 0.5, world_size.y * 0.5);
                let rc0 = RigidCirle::new(10., pos0, 0.69634);
                vec![Box::new(rc0)]
            }
        }
    }

    pub fn get_next_scene(&self) -> Self {
        match self {
            Scene::FallingSquares => Scene::SolarSystem,
            Scene::SolarSystem => Scene::FallingSquares,
        }
    }

    pub fn draw_background(&self) {
        if *self == Scene::FallingSquares {
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
