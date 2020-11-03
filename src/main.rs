mod collision_handling;
mod input_handling;
mod player;

use ggez::*;
use input_handling::handle_keyboard_input;
use nalgebra::Point2;
use player::Player;
use collision_handling::handle_collisions;

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

struct Polygon {
    verts: Vec<Point2<f32>>,
}

impl Polygon {
    fn new(verts: Vec<Point2<f32>>) -> Self {
        Polygon { verts }
    }
}

struct GameMap {
    obstacles: Vec<Polygon>,
    end_area: Polygon,
}

impl GameMap {
    fn new() -> Self {
        let obstacles = vec![
            Polygon::new(vec![
                Point2::new(0.0, 0.0),
                Point2::new(800.0, 0.0),
                Point2::new(800.0, 600.0),
                Point2::new(0.0, 600.0),
            ]),
            Polygon::new(vec![
                Point2::new(250.0, 250.0),
                Point2::new(325.0, 250.0),
                Point2::new(350.0, 350.0),
            ]),
            Polygon::new(vec![
                Point2::new(477.0, 142.0),
                Point2::new(541.0, 189.0),
                Point2::new(449.0, 328.0),
                Point2::new(374.0, 260.0),
                Point2::new(349.0, 211.0),
                Point2::new(428.0, 221.0),
                Point2::new(403.0, 162.0),
            ]),
        ];

        let end_area = Polygon::new(vec![
            Point2::new(600.0, 400.0),
            Point2::new(800.0, 400.0),
            Point2::new(800.0, 600.0),
            Point2::new(600.0, 600.0),
        ]);

        GameMap {obstacles, end_area}
    }
}

struct State {
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
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.player.x, self.player.y],
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

    handle_keyboard_input(ctx, &mut state.player);
    handle_collisions(state);
}
