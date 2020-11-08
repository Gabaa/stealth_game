mod collision_handling;
mod colors;
mod fov;
mod game_map;
mod input_handling;
mod player;
mod polygon;

use collision_handling::apply_physics_movement;
use fov::FieldOfView;
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
    fov: FieldOfView,
    game_map: GameMap,
    player_won: bool,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(),
            fov: FieldOfView::new(),
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

        // TODO: These should re-use the meshes instead of remaking each time
        draw_fov(ctx, &self.fov)?;
        draw_obstacles(ctx, &self.game_map)?;
        draw_end_area(ctx, &self.game_map)?;
        draw_player(ctx, &self.player)?;

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
    state.fov.update(&state.player, &state.game_map);
}

fn draw_fov(ctx: &mut Context, fov: &FieldOfView) -> GameResult<()> {
    println!("FOV Draw: vis area verts: {}", fov.visible_area.verts.len());

    let mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &fov.visible_area.verts,
        colors::VISIBLE_AREA,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

    for vert in &fov.visible_area.verts {
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [vert.x, vert.y],
            10.0,
            0.5,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_obstacles(ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
    for polygon in &game_map.obstacles {
        println!("Obstacle: verts: {}", polygon.verts.len());
        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(3.0),
            &polygon.verts,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_end_area(ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
    println!("end area: verts: {}", game_map.end_area.verts.len());
    let mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &game_map.end_area.verts,
        colors::END_AREA,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}

fn draw_player(ctx: &mut Context, player: &Player) -> GameResult<()> {
    let player_position = player.get_position();
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        [player_position.x, player_position.y],
        player.radius,
        0.5,
        graphics::WHITE,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}
