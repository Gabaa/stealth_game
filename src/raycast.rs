use crate::nalgebra::{distance, Matrix2, Point2, Unit, Vector2};
use crate::polygon::Polygon;

pub struct Ray {
    pub position: Point2<f32>,
    pub direction: Unit<Vector2<f32>>,
}

impl Ray {
    pub fn new(position: Point2<f32>, direction: Unit<Vector2<f32>>) -> Self {
        Ray {
            position,
            direction,
        }
    }

    pub fn rotate(&self, theta: f32) -> Self {
        let rot_matrix = get_rotation_matrix(theta);
        let new_direction = rot_matrix * self.direction.as_ref();
        Ray::new(self.position, Unit::new_normalize(new_direction))
    }
}

fn get_rotation_matrix(theta: f32) -> Matrix2<f32> {
    Matrix2::new(theta.cos(), -theta.sin(), theta.sin(), theta.cos())
}

pub fn raycast(ray: &Ray, polygons: &[Polygon], max_distance: f32) -> Option<Point2<f32>> {
    let mut closest_point_dist = std::f32::MAX;
    let mut closest_point = None;

    for polygon in polygons {
        for (v1, v2) in polygon.edge_iter() {
            if let Some(point) = line_intersection(ray, v1, v2) {
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

/// A ray-line segment intersection algorithm.
///
/// Based on:
///
/// https://stackoverflow.com/questions/14307158/how-do-you-check-for-intersection-between-a-line-segment-and-a-line-ray-emanatin
///
/// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282
fn line_intersection(ray: &Ray, v1: Point2<f32>, v2: Point2<f32>) -> Option<Point2<f32>> {
    let p = ray.position;
    let q = v1;
    let r = ray.direction.into_inner();
    let s = v2 - v1;

    if cross(r, s) == 0.0 {
        if cross(q - p, r) == 0.0 {
            // Lines are collinear
            // t0 = (q − p) · r / (r · r)
            let mut t0 = (q - p).dot(&r) / r.dot(&r);
            // t1 = (q + s − p) · r / (r · r) = t0 + s · r / (r · r)
            let mut t1 = t0 + s.dot(&r) / r.dot(&r);

            if s.dot(&r) < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            }

            if t0 < 1.0 {
                Some(ray.position)
            } else if t1 > 0.0 {
                Some(v1)
            } else {
                None
            }
        } else {
            // Lines are parallel
            None
        }
    } else {
        // t = (q − p) × s / (r × s)
        let t = cross(q - p, s) / cross(r, s);
        // u = (q − p) × r / (r × s)
        let u = cross(q - p, r) / cross(r, s);

        if 0.0 <= u && u <= 1.0 && 0.0 <= t {
            Some(q + u * s)
        } else {
            None
        }
    }
}

fn cross(v: Vector2<f32>, w: Vector2<f32>) -> f32 {
    v.x * w.y - v.y * w.x
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
