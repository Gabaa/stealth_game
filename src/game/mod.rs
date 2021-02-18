pub mod actor;
pub mod collision_handling;
pub mod colors;
pub mod controller;
pub mod drawing;
pub mod fov;
pub mod game_map;
pub mod polygon;
pub mod raycast;

use {
    crate::{
        game::{
            actor::Actor,
            collision_handling::apply_physics_movement,
            controller::{Controller, Patrol},
            drawing::draw_all,
            fov::{ConeFieldOfView, NoFieldOfView},
            game_map::GameMap,
        },
        nalgebra::Point2,
    },
    ggez::{event, Context, GameResult},
    std::boxed::Box,
};

pub struct Game {
    pub actors: Vec<Actor>,
    pub game_map: GameMap,
    pub player_won: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            actors: vec![
                Actor::new(
                    30.0,
                    40.0,
                    Box::new(NoFieldOfView {}),
                    Controller::Player(),
                    1.2,
                ),
                Actor::new(
                    600.0,
                    50.0,
                    Box::new(ConeFieldOfView::new(90.0, 300.0)),
                    Controller::Guard(Patrol {
                        points: vec![
                            Point2::new(604.0, 96.0),
                            Point2::new(279.0, 72.0),
                            Point2::new(65.0, 345.0),
                            Point2::new(326.0, 511.0),
                            Point2::new(659.0, 357.0),
                        ],
                        i: 0,
                    }),
                    1.3,
                ),
            ],
            game_map: GameMap::new(),
            player_won: false,
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        apply_physics_movement(self, ctx);

        if self.player_won {
            println!("You won!");
            event::quit(ctx);
        }

        if was_player_found(&self) {
            println!("Player was discovered...");
            event::quit(ctx);
        }

        for actor in &mut self.actors {
            actor.update_fov(&self.game_map);
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        draw_all(ctx, &self)
    }
}

fn was_player_found(game: &Game) -> bool {
    let mut player_pos_opt = None;

    for actor in &game.actors {
        if actor.is_player() {
            player_pos_opt = Some(actor.pos);
            break;
        }
    }

    match player_pos_opt {
        Some(player_pos) => {
            let mut found = false;

            for actor in &game.actors {
                if actor.is_player() {
                    continue;
                }
                found |= actor.fov.is_inside_fov(actor, &game.game_map, player_pos)
            }

            found
        }
        None => false,
    }
}
