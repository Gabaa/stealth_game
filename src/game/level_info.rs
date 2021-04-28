use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LevelInfo {
    pub player_data: (f32, f32),
    pub guard_data: Vec<((f32, f32), Vec<(f32, f32)>)>,
    pub obstacle_data: Vec<Vec<(f32, f32)>>,
    pub end_area_data: Vec<(f32, f32)>,
}
