use serde::{Deserialize, Serialize};

type Point = (f32, f32);

#[derive(Serialize, Deserialize)]
pub struct LevelInfo {
    pub player_data: Point,
    pub guard_data: Vec<(Point, Vec<Point>)>,
    pub obstacle_data: Vec<Vec<Point>>,
    pub end_area_data: Vec<Point>,
}
