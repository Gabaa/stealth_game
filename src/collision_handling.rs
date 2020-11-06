use crate::nalgebra::{distance, Point2, Vector2};
use crate::player::Player;
use crate::polygon::Polygon;
use crate::State;

pub fn apply_physics_movement(state: &mut State, delta: Vector2<f32>) {
    let next_pos = &mut (state.player.get_position() + delta);
    handle_obstacle_collisions(state, next_pos);
    handle_end_area_intersection(state, next_pos);
    state.player.set_position(*next_pos);
}

fn handle_obstacle_collisions(state: &mut State, next_pos: &mut Point2<f32>) {
    // TODO: Find out if there is a better way than... whatever this is.
    let mut iterations = 0;

    loop {
        if iterations == 100 {
            break;
        }

        let mut changed = false;
        for obstacle in &state.game_map.obstacles {
            changed |= move_out_of_obstacle(obstacle, &state.player, next_pos);
        }

        if !changed {
            break;
        }

        iterations += 1;
    }
}

fn move_out_of_obstacle(obstacle: &Polygon, player: &Player, next_pos: &mut Point2<f32>) -> bool {
    match get_closest_point_on_polygon(obstacle, next_pos) {
        Some(closest_point) => {
            let dist = distance(&closest_point, &next_pos);
            if dist < player.radius {
                let direction =
                    Vector2::new(next_pos.x - closest_point.x, next_pos.y - closest_point.y);
                let unit_direction = direction.normalize();
                let dx = unit_direction.x * (player.radius - dist);
                let dy = unit_direction.y * (player.radius - dist);
                let delta = Vector2::new(dx, dy);
                *next_pos = *next_pos + delta;
                return true;
            }
        }
        None => {}
    }
    false
}

fn get_closest_point_on_polygon(polygon: &Polygon, point: &Point2<f32>) -> Option<Point2<f32>> {
    let mut max_closest = None;
    let mut max_closest_dist = std::f32::MAX;
    for (start, end) in polygon.edge_iter() {
        let closest = &get_closest_point(&start, &end, point);
        let dist = distance(closest, point);
        if dist < max_closest_dist {
            max_closest = Some(*closest);
            max_closest_dist = dist;
        }
    }
    max_closest
}

fn get_closest_point(a: &Point2<f32>, b: &Point2<f32>, p: &Point2<f32>) -> Point2<f32> {
    let ap = p - a;
    let ab = b - a;

    let ap_ab = ap.x * ab.x + ap.y * ab.y;
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let t = (ap_ab / ab2).max(0.0).min(1.0);

    a + ab * t
}

fn handle_end_area_intersection(state: &mut State, next_pos: &Point2<f32>) {
    let end_area = &state.game_map.end_area;

    let n = end_area.verts.len();
    for i in 0..n {
        let a = end_area.verts[i];
        let b = end_area.verts[(i + 1) % n];
        let closest_point = get_closest_point(&a, &b, next_pos);

        let dist = distance(&closest_point, next_pos);
        if dist < state.player.radius {
            state.player_won = true;
        }
    }
}
