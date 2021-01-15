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

pub fn raycast(ray: &Ray, polygons: &[Polygon], max_distance: f32) -> Option<Point2<f32>> {
    let mut closest_point_dist = std::f32::MAX;
    let mut closest_point = None;

    for polygon in polygons {
        for (v1, v2) in polygon.edge_iter() {
            if let Some(point) = line_intersection_v2(ray, v1, v2) {
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
fn _line_intersection_v1(ray: &Ray, v1: Point2<f32>, v2: Point2<f32>) -> Option<Point2<f32>> {
    // Find the smallest angle between ray and line segment
    let (start, end) = if ray.direction.dot(&(v2 - v1)) >= 0.0 {
        (v1, v2)
    } else {
        (v2, v1)
    };

    let edge = end - start;
    let angle = ray.direction.angle(&edge);

    // If lines are parallel, they won't hit
    if angle == 0.0 {
        return None;
    }

    // T2 = (r_dx*(s_py-r_py) + r_dy*(r_px-s_px))/(s_dx*r_dy - s_dy*r_dx)
    let t2 = (ray.direction.x * (start.y - ray.position.y)
        + ray.direction.y * (ray.position.x - start.x))
        / (edge.x * ray.direction.y - edge.y * ray.direction.x);
    // T1 = (s_px+s_dx*T2-r_px)/r_dx
    let t1 = (start.x + edge.x * t2 - ray.position.x) / ray.direction.x;

    if 0.0 < t1 && 0.0 <= t2 && t2 <= 1.0 {
        Some(Point2::new(
            ray.position.x + ray.direction.x * t1,
            ray.position.y + ray.direction.y * t1,
        ))
    } else {
        None
    }
}

/// A ray-line segment intersection algorithm.
///
/// Based on:
///
/// https://stackoverflow.com/questions/14307158/how-do-you-check-for-intersection-between-a-line-segment-and-a-line-ray-emanatin
///
/// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282
fn line_intersection_v2(ray: &Ray, v1: Point2<f32>, v2: Point2<f32>) -> Option<Point2<f32>> {
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

/// A ray-line segment intersection algorithm.
///
/// Based on:
///
/// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
fn _line_intersection_v3(ray: &Ray, v1: Point2<f32>, v2: Point2<f32>) -> Option<Point2<f32>> {
    let r1 = ray.position;
    let r2 = ray.position + ray.direction.into_inner();

    let denom = (r1.x - r2.x) * (v1.y - v2.y) - (r1.y - r2.y) * (v1.x - v2.x);
    if denom == 0.0 {
        return None;
    }

    let t = ((r1.x - v1.x) * (v1.y - v2.y) - (r1.y - v1.y) * (v1.x - v2.x)) / denom;
    if t < 0.0 {
        return None;
    }

    let u = ((r1.x - r2.x) * (r1.y - v1.y) - (r1.y - r2.y) * (r1.x - v1.x)) / denom;
    if u < 0.0 || 1.0 < u {
        return None;
    }

    Some(r1 + (r2 - r1) * t)
}

fn cross(v: Vector2<f32>, w: Vector2<f32>) -> f32 {
    v.x * w.y - v.y * w.x
}

fn get_rotation_matrix(theta: f32) -> Matrix2<f32> {
    Matrix2::new(theta.cos(), -theta.sin(), theta.sin(), theta.cos())
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
