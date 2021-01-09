mod actor;
mod collision_handling;
mod colors;
mod fov;
mod game_map;
mod input_handling;
mod polygon;

use actor::Actor;
use collision_handling::apply_physics_movement;
use fov::FieldOfView;
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
            player: Actor::new(30.0, 40.0, FieldOfView::new()),
            guards: vec![Actor::new(600.0, 50.0, FieldOfView::new())],
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

        // TODO: These should re-use the meshes instead of remaking each time
        draw_all_fov(ctx, &self.player, &self.guards)?;
        draw_obstacles(ctx, &self.game_map)?;
        draw_end_area(ctx, &self.game_map)?;
        draw_actors(ctx, &self.player, &self.guards)?;

        // Present on screen
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
        guard.update_fov_cone(&state.game_map);
    }
    state.player.update_fov(&state.game_map);
}

fn draw_all_fov(ctx: &mut Context, player: &Actor, guards: &Vec<Actor>) -> GameResult<()> {
    for guard in guards {
        draw_fov(ctx, &guard.fov, colors::GUARD_VISIBLE_AREA)?;
    }

    draw_fov(ctx, &player.fov, colors::PLAYER_VISIBLE_AREA)
}

fn draw_fov(ctx: &mut Context, fov: &FieldOfView, color: graphics::Color) -> GameResult<()> {
    let mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &fov.visible_area.verts,
        color,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}

fn draw_obstacles(ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
    for polygon in &game_map.obstacles {
        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(3.0),
            &polygon.verts,
            colors::OBSTACLE,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_end_area(ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
    let mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &game_map.end_area.verts,
        colors::END_AREA,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}

fn draw_actors(ctx: &mut Context, player: &Actor, guards: &Vec<Actor>) -> GameResult<()> {
    for guard in guards {
        draw_actor(ctx, guard)?;
    }

    draw_actor(ctx, player)
}

fn draw_actor(ctx: &mut Context, actor: &Actor) -> GameResult<()> {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        [actor.pos.x, actor.pos.y],
        actor.radius,
        0.5,
        graphics::WHITE,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}
