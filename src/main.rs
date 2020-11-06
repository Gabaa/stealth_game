mod collision_handling;
mod game_map;
mod input_handling;
mod player;
mod polygon;

use collision_handling::apply_physics_movement;
use game_map::GameMap;
use ggez::*;
use input_handling::handle_keyboard_input;
use player::Player;

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
    player: player::Player,
    game_map: GameMap,
    player_won: bool,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(),
            game_map: GameMap::new(),
            player_won: false,
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

        // TODO: don't make a new mesh every draw call, just update an existing one
        // Obstacles
        for polygon in &self.game_map.obstacles {
            let mesh = graphics::Mesh::new_polygon(
                ctx,
                graphics::DrawMode::stroke(3.0),
                &polygon.verts,
                graphics::WHITE,
            )?;

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        let green = graphics::Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        };

        // End area
        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &self.game_map.end_area.verts,
            green,
        )?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Player
        let player_position = self.player.get_position();
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [player_position.x, player_position.y],
            self.player.radius,
            0.5,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Present on screen
        graphics::present(ctx)
    }
}

fn tick(ctx: &mut Context, state: &mut State) {
    if state.player_won {
        println!("You won!");
        event::quit(ctx);
    }

    let delta = handle_keyboard_input(ctx);
    apply_physics_movement(state, delta);
}
