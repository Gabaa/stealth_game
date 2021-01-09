use crate::nalgebra::{Point2, Vector2};
use crate::FieldOfView;
use crate::GameMap;

pub struct Actor {
    pub pos: Point2<f32>,
    pub radius: f32,
    pub direction: Vector2<f32>,
    pub fov: Box<dyn FieldOfView>,
}

impl Actor {
    pub fn new(x: f32, y: f32, fov: Box<dyn FieldOfView>) -> Self {
        Actor {
            pos: Point2::new(x, y),
            radius: 25.0,
            direction: Vector2::new(1.0, 0.0),
            fov: fov,
        }
    }

    pub fn update_fov(&mut self, game_map: &GameMap) {
        self.fov.recalculate(self.pos, self.direction, game_map)
    }
}
