pub mod actor;
pub mod collision_handling;
pub mod controller;
pub mod fov;
pub mod game_map;
pub mod level_info;
pub mod polygon;
pub mod raycast;
pub mod rendering;

use crate::{
    game::{controller::Controller, polygon::Polygon},
    view::ViewEvent,
};

use self::{
    actor::Actor, collision_handling::apply_physics_movement, game_map::GameMap,
    level_info::LevelInfo,
};
use ggez::{nalgebra::Point2, Context};

pub struct Game {
    pub actors: Vec<Actor>,
    pub game_map: GameMap,
    pub player_won: bool,
}

impl Game {
    pub fn new() -> Self {
        let end_area = Polygon::new(vec![
            Point2::new(200.0, 200.0),
            Point2::new(300.0, 200.0),
            Point2::new(300.0, 300.0),
            Point2::new(200.0, 300.0),
        ]);

        Game {
            actors: vec![Actor::new_player(50.0, 50.0)],
            game_map: GameMap::new(vec![], end_area),
            player_won: false,
        }
    }

    pub fn from_level_info(level_info: LevelInfo) -> Self {
        let mut actors = Vec::new();

        // Add player
        let (p_x, p_y) = level_info.player_data;
        actors.push(Actor::new_player(p_x, p_y));

        // Add guards
        for ((g_x, g_y), patrol) in level_info.guard_data {
            let patrol_points = patrol.iter().map(|(x, y)| Point2::new(*x, *y)).collect();
            actors.push(Actor::new_guard(g_x, g_y, patrol_points));
        }

        // Make obstacles
        let obstacles = level_info
            .obstacle_data
            .iter()
            .map(|points| Polygon::new(points.iter().map(|(x, y)| Point2::new(*x, *y)).collect()))
            .collect();

        // Make end area
        let end_area = Polygon::new(
            level_info
                .end_area_data
                .iter()
                .map(|(x, y)| Point2::new(*x, *y))
                .collect(),
        );

        Game {
            actors,
            game_map: GameMap::new(obstacles, end_area),
            player_won: false,
        }
    }

    pub fn to_level_info(&self) -> LevelInfo {
        // Get player data
        let player = self
            .actors
            .iter()
            .find(|actor| actor.is_player())
            .expect("No player found");
        let player_data = (player.pos.x, player.pos.y);

        // Get guard data
        let mut guard_data = Vec::new();
        self.actors
            .iter()
            .filter(|actor| !actor.is_player())
            .for_each(|guard| {
                let pos = (guard.pos.x, guard.pos.y);
                let patrol = match &guard.controller {
                    Controller::Guard(con) => con.points.iter().map(|p| (p.x, p.y)).collect(),
                    _ => unreachable!(),
                };
                guard_data.push((pos, patrol))
            });

        // Get obstacle data
        let obstacle_data = self
            .game_map
            .obstacles
            .iter()
            .map(|p| p.verts.iter().map(|v| (v.x, v.y)).collect())
            .collect();

        // Get end_area_data
        let end_area_data = self
            .game_map
            .end_area
            .verts
            .iter()
            .map(|v| (v.x, v.y))
            .collect();

        LevelInfo {
            player_data,
            guard_data,
            obstacle_data,
            end_area_data,
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) -> Vec<ViewEvent> {
        let mut events = vec![];

        apply_physics_movement(self, ctx);

        if self.player_won {
            println!("You won!");
            events.push(ViewEvent::PopView);
        }

        if was_player_found(self) {
            println!("Player was discovered...");
            events.push(ViewEvent::PopView);
        }

        for actor in &mut self.actors {
            actor.update_fov(&self.game_map);
        }

        events
    }
}

fn was_player_found(game: &mut Game) -> bool {
    let pos = game
        .actors
        .iter()
        .filter(|actor| actor.is_player())
        .map(|actor| actor.pos)
        .next()
        .expect("no player actor found");

    for actor in game.actors.iter_mut().filter(|actor| !actor.is_player()) {
        if actor.fov.is_inside_fov(&game.game_map, pos) {
            actor.discovered_player += 0.015; // approx 60 ticks = 1 second to discover
        } else {
            actor.discovered_player -= 0.015;
            if actor.discovered_player < 0.0 {
                actor.discovered_player = 0.0;
            }
        }
    }

    game.actors
        .iter()
        .any(|actor| actor.discovered_player >= 1.0)
}
