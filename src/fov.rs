use crate::game_map::GameMap;
use crate::nalgebra::{distance_squared, Matrix2, Point, Point2, Unit, Vector2};
use crate::polygon::Polygon;
use std::cmp::Ordering;

pub trait FieldOfView {
    fn get_visible_area(&self) -> &Polygon;
    fn recalculate(&mut self, position: Point2<f32>, direction: Vector2<f32>, game_map: &GameMap);
}

pub struct ConeFieldOfView {
    visible_area: Polygon,
    view_angle_radians: f32,
    view_distance: f32,
}

impl ConeFieldOfView {
    pub fn new(fov_degrees: f32) -> Self {
        ConeFieldOfView {
            visible_area: Polygon::new(vec![
                Point2::new(0.0, 0.0),
                Point2::new(800.0, 0.0),
                Point2::new(800.0, 600.0),
                Point2::new(0.0, 600.0),
            ]),
            view_angle_radians: fov_degrees.to_radians(),
            view_distance: 200.0,
        }
    }
}

impl FieldOfView for ConeFieldOfView {
    fn get_visible_area(&self) -> &Polygon {
        &self.visible_area
    }

    fn recalculate(
        &mut self,
        actor_pos: Point2<f32>,
        actor_direction: Vector2<f32>,
        game_map: &GameMap,
    ) {
        let mut new_verts: Vec<Point2<f32>> = vec![actor_pos];

        for obstacle in &game_map.obstacles {
            for vert in &obstacle.verts {
                let ray_direction = vert - actor_pos;
                let angle = actor_direction.angle(&ray_direction);

                if let Some(ord) = angle.partial_cmp(&self.view_angle_radians) {
                    if ord == Ordering::Greater {
                        continue;
                    }
                }

                let cw_rot_matrix = get_rotation_matrix(0.001);
                let ccw_rot_matrix = get_rotation_matrix(-0.001);
                if let Some(point) = raycast(
                    actor_pos,
                    Unit::new_normalize(ray_direction),
                    &game_map.obstacles,
                    self.view_distance,
                ) {
                    new_verts.push(point);
                };
                if let Some(point) = raycast(
                    actor_pos,
                    Unit::new_normalize(cw_rot_matrix * ray_direction),
                    &game_map.obstacles,
                    self.view_distance,
                ) {
                    new_verts.push(point);
                };
                if let Some(point) = raycast(
                    actor_pos,
                    Unit::new_normalize(ccw_rot_matrix * ray_direction),
                    &game_map.obstacles,
                    self.view_distance,
                ) {
                    new_verts.push(point);
                };
            }
        }

        new_verts.sort_by(|a, b| {
            let a_angle = angle_to_i(a - actor_pos);
            let b_angle = angle_to_i(b - actor_pos);
            a_angle.partial_cmp(&b_angle).unwrap()
        });

        self.visible_area = Polygon::new(new_verts);
    }
}

pub struct GlobalFieldOfView {
    visible_area: Polygon,
    view_distance: f32,
}

impl GlobalFieldOfView {
    pub fn new() -> Self {
        GlobalFieldOfView {
            visible_area: Polygon::new(vec![
                Point2::new(0.0, 0.0),
                Point2::new(800.0, 0.0),
                Point2::new(800.0, 600.0),
                Point2::new(0.0, 600.0),
            ]),
            view_distance: 200.0,
        }
    }
}

impl FieldOfView for GlobalFieldOfView {
    fn get_visible_area(&self) -> &Polygon {
        &self.visible_area
    }

    fn recalculate(&mut self, position: Point2<f32>, _direction: Vector2<f32>, game_map: &GameMap) {
        let mut new_verts: Vec<Point2<f32>> = vec![];

        for obstacle in &game_map.obstacles {
            for vert in &obstacle.verts {
                let direction = vert - position;

                let cw_rot_matrix = get_rotation_matrix(0.001);
                let ccw_rot_matrix = get_rotation_matrix(-0.001);
                if let Some(point) = raycast(
                    position,
                    Unit::new_normalize(direction),
                    &game_map.obstacles,
                    self.view_distance,
                ) {
                    new_verts.push(point);
                };
                if let Some(point) = raycast(
                    position,
                    Unit::new_normalize(cw_rot_matrix * direction),
                    &game_map.obstacles,
                    self.view_distance,
                ) {
                    new_verts.push(point);
                };
                if let Some(point) = raycast(
                    position,
                    Unit::new_normalize(ccw_rot_matrix * direction),
                    &game_map.obstacles,
                    self.view_distance,
                ) {
                    new_verts.push(point);
                };
            }
        }

        new_verts.sort_by(|a, b| {
            let a_angle = angle_to_i(a - position);
            let b_angle = angle_to_i(b - position);
            a_angle.partial_cmp(&b_angle).unwrap()
        });

        self.visible_area = Polygon::new(new_verts);
    }
}

fn get_rotation_matrix(theta: f32) -> Matrix2<f32> {
    Matrix2::new(theta.cos(), -theta.sin(), theta.sin(), theta.cos())
}

fn angle_to_i(v: Vector2<f32>) -> f32 {
    let i = Vector2::new(1.0, 0.0);

    let a_dot = i.dot(&v);
    let a_det = i.x * v.y - i.y * v.x;

    a_dot.atan2(a_det)
}

fn raycast(
    pos: Point2<f32>,
    dir: Unit<Vector2<f32>>,
    polygons: &[Polygon],
    max_dist: f32,
) -> Option<Point2<f32>> {
    let mut closest_point_dist = std::f32::MAX;
    let mut closest_point: Option<Point2<f32>> = None;

    if max_dist > 0.0 {
        closest_point_dist = max_dist.powi(2);
        closest_point = Some(Point::from(dir.as_ref() * max_dist));
    }

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

            if 0.0 < t1 && 0.0 <= t2 && t2 <= 1.0 {
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
    use super::{raycast, Point2, Polygon, Unit, Vector2};

    #[test]
    fn hit_nothing() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Unit::new_normalize(Vector2::new(1.0, 0.0));
        let polygons = vec![];
        let hit = raycast(pos, dir, &polygons, 0.0);
        assert!(hit.is_none());
    }

    #[test]
    fn hit_something() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Unit::new_normalize(Vector2::new(1.0, 0.0));
        let polygons = vec![Polygon::new(vec![
            Point2::new(1.0, -1.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 1.0),
            Point2::new(2.0, -1.0),
        ])];
        let hit = raycast(pos, dir, &polygons, 0.0);
        match hit {
            Some(pos) => assert_eq!(pos, Point2::new(1.0, 0.0)),
            None => panic!("did not hit anything"),
        }
    }
}
