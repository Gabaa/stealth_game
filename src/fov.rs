use crate::game_map::GameMap;
use crate::nalgebra::{Point2, Unit, Vector2};
use crate::polygon::Polygon;
use crate::raycast::{raycast, Ray};
use std::cmp::Ordering;

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
            let a_angle = signed_angle(a - actor_pos, actor_direction.into_inner());
            let b_angle = signed_angle(b - actor_pos, actor_direction.into_inner());
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
            let a_angle = signed_angle(a - position, Vector2::new(1.0, 0.0));
            let b_angle = signed_angle(b - position, Vector2::new(1.0, 0.0));
            a_angle.partial_cmp(&b_angle).unwrap()
        });

        self.visible_area = Polygon::new(new_verts);
    }
}

fn signed_angle(v1: Vector2<f32>, v2: Vector2<f32>) -> f32 {
    let a_dot = v2.dot(&v1);
    let a_det = v2.x * v1.y - v2.y * v1.x;

    a_dot.atan2(a_det)
}
