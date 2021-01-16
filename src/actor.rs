use crate::fov::FieldOfView;
use crate::game_map::GameMap;
use crate::nalgebra::{Point2, Unit, Vector2};

pub struct Actor {
    pub pos: Point2<f32>,
    pub radius: f32,
    pub direction: Unit<Vector2<f32>>,
    pub fov: Box<dyn FieldOfView>,
}

impl Actor {
    pub fn new(x: f32, y: f32, fov: Box<dyn FieldOfView>) -> Self {
        Actor {
            pos: Point2::new(x, y),
            radius: 25.0,
            direction: Unit::new_normalize(Vector2::new(1.0, 0.0)),
            fov,
        }
    }

    pub fn update_fov(&mut self, game_map: &GameMap) {
        self.fov.recalculate(self.pos, self.direction, game_map)
    }
}
