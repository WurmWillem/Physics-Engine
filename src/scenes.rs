use std::ops::RangeInclusive;

use macroquad::prelude::*;

use crate::{
    engine::Variables, rigid_body::RigidBody, rigid_circle::RigidCircle,
    rigid_rectangle::RigidSquare, rigid_spring::RigidSpring, SCREEN_SIZE, SCREEN_X_INCREASE,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    FallingRectangles,
    BouncingCircles,
    RectAndCircle,
    Spring,
}
impl Scene {
    pub fn get_world_size(&self) -> Vec2 {
        match self {
            Scene::FallingRectangles => vec2(60. * SCREEN_X_INCREASE, 60.),
            Scene::BouncingCircles => vec2(40. * SCREEN_X_INCREASE, 40.),
            Scene::RectAndCircle => vec2(60. * SCREEN_X_INCREASE, 60.),
            Scene::Spring => vec2(40. * SCREEN_X_INCREASE, 40.),
        }
    }
    pub fn get_rigid_bodies(&self) -> Vec<Box<dyn RigidBody>> {
        let world_size = self.get_world_size();
        match self {
            Scene::FallingRectangles => {
                let pos0 = vec2(world_size.x * 0.45, world_size.y * 0.5);
                let pos1 = vec2(world_size.x * 0.55, world_size.y * 0.5);

                let rs0 = RigidSquare::new(10., pos0, vec2(2., 2.));
                let rs1 = RigidSquare::new(100., pos1, vec2(2., 2.));
                vec![Box::new(rs0), Box::new(rs1)]
            }
            Scene::BouncingCircles => {
                let radius_0 = 1.;
                let radius_1 = 2.;
                let radius_2 = 3.;
                let radius_3 = 4.;
                let pos0 = vec2(world_size.x * 0.1, world_size.y * 0.8 + radius_0);
                let pos1 = vec2(world_size.x * 0.3, world_size.y * 0.8 + radius_1);
                let pos2 = vec2(world_size.x * 0.6, world_size.y * 0.8 + radius_2);
                let pos3 = vec2(world_size.x * 0.9, world_size.y * 0.8 + radius_3);

                let rb0 = RigidCircle::new(1., pos0, radius_0);
                let rb1 = RigidCircle::new(4., pos1, radius_1);
                let rb2 = RigidCircle::new(9., pos2, radius_2);
                let rb3 = RigidCircle::new(16., pos3, radius_3);
                vec![Box::new(rb0), Box::new(rb1), Box::new(rb2), Box::new(rb3)]
            }
            Scene::RectAndCircle => {
                let size_rs = vec2(6., 6.);
                let pos_rs = vec2(world_size.x * 0.5 - size_rs.x * 0.5, world_size.y * 0.5);

                let rs0 = RigidSquare::new(1., pos_rs, size_rs);
                let radius_0 = 1.;
                let radius_1 = 2.;
                let radius_2 = 3.;
                let radius_3 = 4.;
                let pos0 = vec2(world_size.x * 0.1, world_size.y * 0.8 + radius_0);
                let pos1 = vec2(world_size.x * 0.3, world_size.y * 0.8 + radius_1);
                let pos2 = vec2(world_size.x * 0.6, world_size.y * 0.8 + radius_2);
                let pos3 = vec2(world_size.x * 0.9, world_size.y * 0.8 + radius_3);

                let rb0 = RigidCircle::new(1., pos0, radius_0);
                let rb1 = RigidCircle::new(4., pos1, radius_1);
                let rb2 = RigidCircle::new(9., pos2, radius_2);
                let rb3 = RigidCircle::new(16., pos3, radius_3);
                vec![
                    Box::new(rs0),
                    Box::new(rb0),
                    Box::new(rb1),
                    Box::new(rb2),
                    Box::new(rb3),
                ]
            }
            Scene::Spring => {
                let pos = vec2(world_size.x * 0.5 - 10., world_size.y * 0.5);
                let spring = RigidSpring::new(1., pos, vec2(30., 3.));
                vec![Box::new(spring)]
            }
        }
    }
    pub fn get_next_scene(&self) -> Self {
        match self {
            Scene::FallingRectangles => Scene::BouncingCircles,
            Scene::BouncingCircles => Scene::RectAndCircle,
            Scene::RectAndCircle => Scene::Spring,
            Scene::Spring => Scene::FallingRectangles,
        }
    }
    pub fn get_variables(&self) -> Variables {
        match self {
            Scene::FallingRectangles => Variables::new(Some(0.), Some(1.)),
            Scene::BouncingCircles => Variables::new(Some(9.81), Some(0.)),
            Scene::RectAndCircle => Variables::new(Some(9.81), Some(0.)),
            Scene::Spring => Variables::new(None, None),
        }
    }
    pub fn get_c_range(&self) -> RangeInclusive<f32> {
        match self {
            Scene::FallingRectangles => (-1.)..=30.,
            Scene::BouncingCircles => (-0.01)..=1.,
            Scene::RectAndCircle => (-0.01)..=1.,
            Scene::Spring => (-0.01)..=1.,
        }
    }
    pub fn get_c_default(&self) -> f32 {
        match self {
            Scene::FallingRectangles => 1.,
            Scene::BouncingCircles => 0.01,
            Scene::RectAndCircle => 0.01,
            Scene::Spring => 0.01,
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
