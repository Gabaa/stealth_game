use crate::nalgebra::Point2;

pub struct Player {
    pos: Point2<f32>,
    pub radius: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Point2::new(30.0, 40.0),
            radius: 25.0,
        }
    }

    pub fn get_position(&self) -> Point2<f32> {
        self.pos
    }

    pub fn set_position(&mut self, new_pos: Point2<f32>) {
        self.pos = new_pos;
    }
}
