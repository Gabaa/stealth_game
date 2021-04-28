use crate::game::polygon::Polygon;

pub struct GameMap {
    pub obstacles: Vec<Polygon>,
    pub end_area: Polygon,
}

impl GameMap {
    pub fn new(obstacles: Vec<Polygon>, end_area: Polygon) -> Self {
        GameMap {
            obstacles,
            end_area,
        }
    }
}
