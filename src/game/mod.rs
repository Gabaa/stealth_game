pub mod actor;
pub mod collision_handling;
pub mod controller;
pub mod fov;
pub mod game_map;
pub mod polygon;
pub mod raycast;
pub mod renderer;

use self::{
    actor::Actor, collision_handling::apply_physics_movement, game_map::GameMap, renderer::Renderer,
};
use ggez::{event, Context, GameResult};

pub struct Game {
    pub actors: Vec<Actor>,
    pub game_map: GameMap,
    pub player_won: bool,
    renderer: Renderer,
}

impl Game {
    pub fn new() -> Self {
        let actors = vec![Actor::new_player(30.0, 40.0), Actor::new_guard(600.0, 50.0)];

        Game {
            actors,
            game_map: GameMap::new(),
            player_won: false,
            renderer: Renderer::new(),
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
        self.renderer.render(ctx, &self)
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
