use crate::nalgebra::Point2;
use crate::FieldOfView;
use crate::GameMap;

pub struct Actor {
    pub pos: Point2<f32>,
    pub radius: f32,
    pub fov: FieldOfView,
}

impl Actor {
    pub fn new() -> Self {
        Actor {
            pos: Point2::new(30.0, 40.0),
            radius: 25.0,
            fov: FieldOfView::new(),
        }
    }

    pub fn update_fov(&mut self, game_map: &GameMap) {
        self.fov.update(self.pos, game_map)
    }
}
