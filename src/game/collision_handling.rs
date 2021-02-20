use {
    crate::game::{
        actor::Actor, controller::Controller, game_map::GameMap, polygon::Polygon, Game,
    },
    ggez::{
        nalgebra::{distance, Point2, Unit, Vector2},
        Context,
    },
};

pub fn apply_physics_movement(game: &mut Game, ctx: &Context) {
    for actor in &mut game.actors {
        let delta = actor.get_next_movement(ctx);

        let next_pos = &mut (actor.pos + delta);
        if delta.magnitude() > 0.0 {
            actor.direction = Unit::new_normalize(delta);
        }

        handle_obstacle_collisions(&game.game_map, actor, next_pos);

        if let Controller::Player() = actor.controller {
            game.player_won = did_player_win(&game.game_map, &actor, *next_pos);
        }

        actor.pos = *next_pos;
    }
}

fn handle_obstacle_collisions(game_map: &GameMap, actor: &mut Actor, next_pos: &mut Point2<f32>) {
    // TODO: Find out if there is a better way than... whatever this is.
    let mut iterations = 0;

    loop {
        iterations += 1;

        let mut changed = false;
        for obstacle in &game_map.obstacles {
            changed |= move_out_of_obstacle(obstacle, actor, next_pos);
        }

        if iterations == 100 || !changed {
            break;
        }
    }
}

fn move_out_of_obstacle(obstacle: &Polygon, player: &Actor, next_pos: &mut Point2<f32>) -> bool {
    if let Some(closest_point) = get_closest_point_on_polygon(obstacle, *next_pos) {
        let dist = distance(&closest_point, &next_pos);
        if dist < player.radius {
            let direction = *next_pos - closest_point;
            let unit_direction = direction.normalize();
            let dx = unit_direction.x * (player.radius - dist);
            let dy = unit_direction.y * (player.radius - dist);
            let delta = Vector2::new(dx, dy);
            *next_pos += delta;
            return true;
        }
    }
    false
}

fn get_closest_point_on_polygon(polygon: &Polygon, point: Point2<f32>) -> Option<Point2<f32>> {
    let mut max_closest = None;
    let mut max_closest_dist = std::f32::MAX;
    for (start, end) in polygon.edge_iter() {
        let closest = &get_closest_point(start, end, point);
        let dist = distance(closest, &point);
        if dist < max_closest_dist {
            max_closest = Some(*closest);
            max_closest_dist = dist;
        }
    }
    max_closest
}

fn get_closest_point(a: Point2<f32>, b: Point2<f32>, p: Point2<f32>) -> Point2<f32> {
    let ap = p - a;
    let ab = b - a;

    let ap_ab = ap.x * ab.x + ap.y * ab.y;
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let t = (ap_ab / ab2).max(0.0).min(1.0);

    a + ab * t
}

fn did_player_win(game_map: &GameMap, player: &Actor, next_pos: Point2<f32>) -> bool {
    let end_area = &game_map.end_area;

    for (a, b) in end_area.edge_iter() {
        let closest_point = get_closest_point(a, b, next_pos);

        let dist = distance(&closest_point, &next_pos);
        if dist < player.radius {
            return true;
        }
    }

    false
}
