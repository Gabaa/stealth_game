mod actor;
mod collision_handling;
mod colors;
mod drawing;
mod fov;
mod game_map;
mod input_handling;
mod polygon;
mod raycast;

use actor::Actor;
use collision_handling::apply_physics_movement;
use drawing::draw_all;
use fov::ConeFieldOfView;
use game_map::GameMap;
use ggez::*;
use input_handling::handle_keyboard_input;

fn main() {
    let mut state = State::new();

    let conf = conf::Conf {
        window_mode: conf::WindowMode::default(),
        window_setup: conf::WindowSetup::default().title("Stealth Game!!!"),
        backend: conf::Backend::default(),
        modules: conf::ModuleConf::default(),
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("stealth_game", "Gabaa")
        .conf(conf)
        .build()
        .unwrap();

    match event::run(ctx, event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

pub struct State {
    player: Actor,
    guards: Vec<Actor>,
    game_map: GameMap,
    player_won: bool,
    player_found: bool,
}

impl State {
    fn new() -> Self {
        State {
            player: Actor::new(30.0, 40.0, Box::new(ConeFieldOfView::new(360.0, 200.0))),
            guards: vec![Actor::new(
                600.0,
                50.0,
                Box::new(ConeFieldOfView::new(90.0, 200.0)),
            )],
            game_map: GameMap::new(),
            player_won: false,
            player_found: false,
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            tick(ctx, self);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        draw_all(ctx, &self)?;
        graphics::present(ctx)
    }
}

fn tick(ctx: &mut Context, state: &mut State) {
    if state.player_won {
        println!("You won!");
        event::quit(ctx);
    }

    if state.player_found {
        println!("Player was discovered...");
        event::quit(ctx);
    }

    let delta = handle_keyboard_input(ctx);
    apply_physics_movement(state, delta);

    for guard in &mut state.guards {
        guard.update_fov(&state.game_map);
    }
    state.player.update_fov(&state.game_map);
}
