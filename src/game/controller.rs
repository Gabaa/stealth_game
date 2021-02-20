use ggez::nalgebra::Point2;

pub enum Controller {
    Player(),
    Guard(Patrol),
}

pub struct Patrol {
    pub points: Vec<Point2<f32>>,
    pub i: usize,
}
