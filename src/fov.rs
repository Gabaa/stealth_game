use crate::game_map::GameMap;
use crate::nalgebra::{distance, Matrix2, Point2, Unit, Vector2};
use crate::polygon::Polygon;
use std::cmp::Ordering;
use std::f32::consts::FRAC_PI_2;

pub trait FieldOfView {
    fn get_visible_area(&self) -> &Polygon;
    fn recalculate(
        &mut self,
        position: Point2<f32>,
        direction: Unit<Vector2<f32>>,
        game_map: &GameMap,
    );
}

pub struct ConeFieldOfView {
    visible_area: Polygon,
    view_angle_radians: f32,
    view_distance: f32,
}

impl ConeFieldOfView {
    pub fn new(fov_degrees: f32) -> Self {
        ConeFieldOfView {
            visible_area: Polygon::new(vec![]),
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
        actor_direction: Unit<Vector2<f32>>,
        game_map: &GameMap,
    ) {
        let mut new_verts: Vec<Point2<f32>> = vec![actor_pos];

        // Shoot a ray straight forward, and at each edge of their vision
        // TODO: Shoot more rays, so the shape is more understandable
        let ray = Ray::new(actor_pos, actor_direction);
        new_verts.push(
            match raycast(&ray, &game_map.obstacles, self.view_distance) {
                Some(hit_pos) => hit_pos,
                None => actor_pos + ray.direction.as_ref() * self.view_distance,
            },
        );
        let rotate_rad = self.view_angle_radians / 2.0;
        let cw_edge_ray = ray.rotate(rotate_rad);
        new_verts.push(
            match raycast(&cw_edge_ray, &game_map.obstacles, self.view_distance) {
                Some(hit_pos) => hit_pos,
                None => actor_pos + cw_edge_ray.direction.as_ref() * self.view_distance,
            },
        );
        let ccw_edge_ray = ray.rotate(-rotate_rad);
        new_verts.push(
            match raycast(&ccw_edge_ray, &game_map.obstacles, self.view_distance) {
                Some(hit_pos) => hit_pos,
                None => actor_pos + ccw_edge_ray.direction.as_ref() * self.view_distance,
            },
        );

        for obstacle in &game_map.obstacles {
            for vert in &obstacle.verts {
                let ray_direction = vert - actor_pos;
                let angle = actor_direction.angle(&ray_direction);

                if let Some(ord) = angle.partial_cmp(&(self.view_angle_radians / 2.0)) {
                    if ord == Ordering::Greater {
                        continue;
                    }
                }

                let ray = Ray::new(actor_pos, Unit::new_normalize(ray_direction));
                if let Some(point) = raycast(&ray, &game_map.obstacles, self.view_distance) {
                    new_verts.push(point);
                };

                let cw_rot_ray = ray.rotate(0.001);
                if let Some(point) = raycast(&cw_rot_ray, &game_map.obstacles, self.view_distance) {
                    new_verts.push(point);
                };

                let ccw_rot_ray = ray.rotate(-0.001);
                if let Some(point) = raycast(&ccw_rot_ray, &game_map.obstacles, self.view_distance)
                {
                    new_verts.push(point);
                };
            }
        }

        new_verts.sort_by(|a, b| {
            let a_angle = signed_angle(&(a - actor_pos), actor_direction.as_ref());
            let b_angle = signed_angle(&(b - actor_pos), actor_direction.as_ref());
            match a_angle.partial_cmp(&b_angle) {
                Some(ord) => ord,
                None => panic!("Cannot sort the FOV verts, {} and {}", a, b),
            }
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

    fn recalculate(
        &mut self,
        position: Point2<f32>,
        _direction: Unit<Vector2<f32>>,
        game_map: &GameMap,
    ) {
        let mut new_verts: Vec<Point2<f32>> = vec![];

        for obstacle in &game_map.obstacles {
            for vert in &obstacle.verts {
                let direction = vert - position;

                let ray = Ray::new(position, Unit::new_normalize(direction));
                if let Some(point) = raycast(&ray, &game_map.obstacles, 0.0) {
                    new_verts.push(point);
                };

                let cw_rot_ray = ray.rotate(0.001);
                if let Some(point) = raycast(&cw_rot_ray, &game_map.obstacles, 0.0) {
                    new_verts.push(point);
                };

                let ccw_rot_ray = ray.rotate(-0.001);
                if let Some(point) = raycast(&ccw_rot_ray, &game_map.obstacles, 0.0) {
                    new_verts.push(point);
                };
            }
        }

        new_verts.sort_by(|a, b| {
            let a_angle = signed_angle(&(a - position), &Vector2::new(1.0, 0.0));
            let b_angle = signed_angle(&(b - position), &Vector2::new(1.0, 0.0));
            a_angle.partial_cmp(&b_angle).unwrap()
        });

        self.visible_area = Polygon::new(new_verts);
    }
}

fn get_rotation_matrix(theta: f32) -> Matrix2<f32> {
    Matrix2::new(theta.cos(), -theta.sin(), theta.sin(), theta.cos())
}

fn signed_angle(v1: &Vector2<f32>, v2: &Vector2<f32>) -> f32 {
    let a_dot = v2.dot(&v1);
    let a_det = v2.x * v1.y - v2.y * v1.x;

    a_dot.atan2(a_det)
}

struct Ray {
    position: Point2<f32>,
    direction: Unit<Vector2<f32>>,
}

impl Ray {
    fn new(position: Point2<f32>, direction: Unit<Vector2<f32>>) -> Self {
        Ray {
            position,
            direction,
        }
    }

    fn rotate(&self, theta: f32) -> Self {
        let rot_matrix = get_rotation_matrix(theta);
        let new_direction = rot_matrix * self.direction.as_ref();
        Ray::new(self.position, Unit::new_normalize(new_direction))
    }
}

fn raycast(ray: &Ray, polygons: &[Polygon], max_distance: f32) -> Option<Point2<f32>> {
    let mut closest_point_dist = std::f32::MAX;
    let mut closest_point = None;

    for polygon in polygons {
        for (v1, v2) in polygon.edge_iter() {
            // Find the smallest angle between ray and line segment
            let (start, end) = if is_angle_right_or_less(v1, v2, *ray.direction) {
                (v1, v2)
            } else {
                (v2, v1)
            };

            let edge = end - start;
            let angle = ray.direction.angle(&edge);

            // If lines are parallel, they won't hit
            if angle == 0.0 {
                continue;
            }

            // TODO: doesn't work with rays going straight up (or down???).
            // Maybe this can help???
            // https://stackoverflow.com/questions/14307158/how-do-you-check-for-intersection-between-a-line-segment-and-a-line-ray-emanatin

            // TODO: Clean this up!
            // T2 = (r_dx*(s_py-r_py) + r_dy*(r_px-s_px))/(s_dx*r_dy - s_dy*r_dx)
            let t2 = (ray.direction.x * (start.y - ray.position.y)
                + ray.direction.y * (ray.position.x - start.x))
                / (edge.x * ray.direction.y - edge.y * ray.direction.x);
            // T1 = (s_px+s_dx*T2-r_px)/r_dx
            let t1 = (start.x + edge.x * t2 - ray.position.x) / ray.direction.x;

            if 0.0 < t1 && 0.0 <= t2 && t2 <= 1.0 {
                let point = Point2::new(
                    ray.position.x + ray.direction.x * t1,
                    ray.position.y + ray.direction.y * t1,
                );
                let new_dist = distance(&point, &ray.position);
                if new_dist < closest_point_dist {
                    closest_point = Some(point);
                    closest_point_dist = new_dist;
                }
            }
        }
    }

    if 0.0 < max_distance && max_distance < closest_point_dist {
        None
    } else {
        closest_point
    }
}

fn is_angle_right_or_less(v1: Point2<f32>, v2: Point2<f32>, ray_direction: Vector2<f32>) -> bool {
    ray_direction.angle(&(v2 - v1)) <= FRAC_PI_2
}

#[cfg(test)]
mod raycast_tests {
    use super::{raycast, Point2, Polygon, Ray, Unit, Vector2};

    #[test]
    fn hit_nothing() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let ray = Ray::new(pos, Unit::new_normalize(dir));
        let polygons = vec![];
        let hit = raycast(&ray, &polygons, 0.0);
        assert!(hit.is_none());
    }

    #[test]
    fn hit_square_edge() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let ray = Ray::new(pos, Unit::new_normalize(dir));

        let polygons = vec![Polygon::new(vec![
            Point2::new(1.0, -1.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 1.0),
            Point2::new(2.0, -1.0),
        ])];
        let hit = raycast(&ray, &polygons, 0.0);
        match hit {
            Some(hit_pos) => assert_eq!(hit_pos, Point2::new(1.0, 0.0)),
            None => panic!("did not hit anything"),
        }
    }

    #[test]
    fn hit_square_corner() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let ray = Ray::new(pos, Unit::new_normalize(dir));

        let verts = vec![
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 1.0),
            Point2::new(2.0, 0.0),
            Point2::new(1.0, 0.0),
        ];
        let polygons = vec![Polygon::new(verts)];

        let hit = raycast(&ray, &polygons, 0.0);
        match hit {
            Some(hit_pos) => assert_eq!(hit_pos, Point2::new(1.0, 0.0)),
            None => panic!("did not hit anything"),
        }
    }

    #[test]
    fn hit_nothing_distance() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let ray = Ray::new(pos, Unit::new_normalize(dir));

        let polygons = vec![Polygon::new(vec![
            Point2::new(6.0, -1.0),
            Point2::new(6.0, 1.0),
            Point2::new(7.0, 1.0),
            Point2::new(7.0, -1.0),
        ])];

        let hit = raycast(&ray, &polygons, 5.0);
        assert!(hit.is_none());
    }

    #[test]
    fn hit_triangle_corner() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(1.0, 0.0);
        let ray = Ray::new(pos, Unit::new_normalize(dir));

        let verts = vec![
            Point2::new(1.0, 0.0),
            Point2::new(0.5, 0.5),
            Point2::new(1.5, 0.5),
        ];
        let polygons = vec![Polygon::new(verts)];

        let hit = raycast(&ray, &polygons, 0.0);
        match hit {
            Some(hit_pos) => assert_eq!(hit_pos, Point2::new(1.0, 0.0)),
            None => panic!("did not hit triangle"),
        }
    }

    #[test]
    fn hit_upwards() {
        let pos = Point2::new(0.0, 0.0);
        let dir = Vector2::new(0.0, 1.0);
        let ray = Ray::new(pos, Unit::new_normalize(dir));

        let polygons = vec![Polygon::new(vec![
            Point2::new(-1.0, 1.0),
            Point2::new(1.0, 1.0),
            Point2::new(1.0, 2.0),
            Point2::new(-1.0, 2.0),
        ])];
        let hit = raycast(&ray, &polygons, 0.0);
        match hit {
            Some(hit_pos) => assert_eq!(hit_pos, Point2::new(0.0, 1.0)),
            None => panic!("did not hit anything"),
        }
    }
}
