use crate::game_map::GameMap;
use crate::nalgebra::{distance_squared, Point2, Vector2};
use crate::player::Player;
use crate::polygon::Polygon;

pub struct FieldOfView {
    pub visible_area: Polygon,
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

    pub fn update(&mut self, player: &Player, game_map: &GameMap) {
        let mut new_verts: Vec<Point2<f32>> = vec![];
        let pos = player.get_position();

        for obstacle in &game_map.obstacles {
            for vert in &obstacle.verts {
                let direction = vert - pos;
                match raycast(pos, direction, &game_map.obstacles) {
                    Some(point) => {
                        new_verts.push(point);
                    }
                    None => {}
                };
            }
        }

        new_verts.sort_by(|a, b| {
            let r = Vector2::new(1.0, 0.0);
            let a_angle = r.angle(&a.coords);
            let b_angle = r.angle(&b.coords);
            a_angle.partial_cmp(&b_angle).unwrap()
        });

        self.visible_area = Polygon::new(new_verts);
    }
}

fn raycast(pos: Point2<f32>, dir: Vector2<f32>, polygons: &Vec<Polygon>) -> Option<Point2<f32>> {
    let dir = dir.normalize();
    let mut closest_point_dist = std::f32::MAX;
    let mut closest_point = None;
    for polygon in polygons {
        for (start, end) in polygon.edge_iter() {
            let edge_dir = end - start;
            let angle = dir.angle(&edge_dir);

            if angle == 0.0 {
                break;
            }

            // T2 = (r_dx*(s_py-r_py) + r_dy*(r_px-s_px))/(s_dx*r_dy - s_dy*r_dx)
            let t2 = (dir.x * (start.y - pos.y) + dir.y * (pos.x - start.x))
                / (edge_dir.x * dir.y - edge_dir.y * dir.x);
            // T1 = (s_px+s_dx*T2-r_px)/r_dx
            let t1 = (start.x + edge_dir.x * t2 - pos.x) / dir.x;

            if 0.0 < t1 && 0.0 < t2 && t2 < 1.0 {
                let point = Point2::new(pos.x + dir.x * t1, pos.y + dir.y * t1);
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

#[cfg(test)]
mod raycast_tests {
    use super::{raycast, Point2, Polygon, Vector2};

    #[test]
    fn hit_nothing() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let polygons = vec![];
        let hit = raycast(pos, dir, &polygons);
        assert!(hit.is_none());
    }

    #[test]
    fn hit_something() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let polygons = vec![Polygon::new(vec![
            Point2::new(1.0, -1.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 1.0),
            Point2::new(2.0, -1.0),
        ])];
        let hit = raycast(pos, dir, &polygons);
        match hit {
            Some(pos) => assert_eq!(pos, Point2::new(1.0, 0.0)),
            None => panic!("did not hit anything"),
        }
    }
}
