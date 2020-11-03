use crate::nalgebra::{distance, Point2, Unit};
use crate::player::Player;
use crate::State;

pub fn handle_collisions(state: &mut State) {
    state.player.apply_movement();
    handle_obstacle_collisions(state);
    handle_end_area_intersection(state);
}

fn handle_obstacle_collisions(state: &mut State) -> (f32, f32) {
    let Player { x, y, .. } = state.player;
    let mut dx = state.player.dx;
    let mut dy = state.player.dy;

    for obstacle in &state.game_map.obstacles {
        let n = obstacle.verts.len();
        for i in 0..n {
            let a = obstacle.verts[i];
            let b = obstacle.verts[(i + 1) % n];
            let center = Point2::new(x + dx, y + dy);
            let closest_point = get_closest_point(a, b, center);

            let dist = distance(&closest_point, &center);
            if dist < state.player.radius {
                let direction = center - closest_point;
                let unit_direction = Unit::new_normalize(direction).into_inner();
                dx += unit_direction.x * (state.player.radius - dist) * 0.5;
                dy += unit_direction.y * (state.player.radius - dist) * 0.5;
            }
        }
    }

    (dx, dy)
}

fn handle_end_area_intersection(state: &mut State) {
    let end_area = &state.game_map.end_area;
    let position = Point2::new(state.player.x, state.player.y);

    let n = end_area.verts.len();
    for i in 0..n {
        let a = end_area.verts[i];
        let b = end_area.verts[(i + 1) % n];
        let closest_point = get_closest_point(a, b, position);

        let dist = distance(&closest_point, &position);
        if dist < state.player.radius {
            state.player_won = true;
        }
    }
}

fn get_closest_point(a: Point2<f32>, b: Point2<f32>, p: Point2<f32>) -> Point2<f32> {
    let ap = p - a;
    let ab = b - a;

    let ap_ab = ap.x * ab.x + ap.y * ab.y;
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let t = (ap_ab / ab2).max(0.0).min(1.0);

    a + ab * t
}
