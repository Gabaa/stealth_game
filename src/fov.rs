use crate::game_map::GameMap;
use crate::nalgebra::{distance_squared, Point2, Vector2};
use crate::player::Player;
use crate::polygon::Polygon;

pub struct FieldOfView {
    visible_area: Polygon,
}

impl FieldOfView {
    pub fn new() -> Self {
        FieldOfView {
            visible_area: Polygon::new(vec![
                Point2::new(0.0, 0.0),
                Point2::new(800.0, 0.0),
                Point2::new(800.0, 600.0),
                Point2::new(0.0, 600.0),
            ]),
        }
    }

    pub fn get_area(&self) -> &Polygon {
        &self.visible_area
    }

    pub fn update(&mut self, player: &Player, game_map: &GameMap) {
        println!("Update!");
        let mut new_verts: Vec<Point2<f32>> = vec![];
        let pos = player.get_position();

        for obstacle in &game_map.obstacles {
            for vert in &obstacle.verts {
                println!("Checking a vert");
                match raycast(pos, vert - pos, game_map) {
                    Some(point) => {
                        new_verts.push(point);
                        println!("{:?}", point);
                    },
                    None => {}
                };
            }
        }

        self.visible_area = Polygon::new(new_verts);
    }
}

fn raycast(pos: Point2<f32>, direction: Vector2<f32>, game_map: &GameMap) -> Option<Point2<f32>> {
    // TODO: Isn't really working, fix pls
    let direction = direction.normalize();
    let mut closest_point_dist = std::f32::MAX;
    let mut closest_point = None;
    for obstacle in &game_map.obstacles {
        for (start, end) in obstacle.edge_iter() {
            let seg_direction = (end - start).normalize();
            if direction == seg_direction {
                break;
            }

            // T2 = (r_dx*(s_py-r_py) + r_dy*(r_px-s_px))/(s_dx*r_dy - s_dy*r_dx)
            let t2 = (direction.x * (start.y - pos.y) + direction.y * (pos.x - start.x))
                / (seg_direction.x * direction.y - seg_direction.y * direction.x);
            // T1 = (s_px+s_dx*T2-r_px)/r_dx
            let t1 = (start.x + seg_direction.x * t2 - pos.x) / direction.x;

            if 0.0 < t1 && 0.0 < t2 && t2 < 1.0 {
                let point = Point2::new(pos.x + direction.x * t1, pos.y + direction.y * t1);
                let new_dist = distance_squared(&point, &pos);
                if new_dist < closest_point_dist {
                    closest_point = Some(point);
                    closest_point_dist = new_dist;
                }
            }
        }
    }

    closest_point
}
