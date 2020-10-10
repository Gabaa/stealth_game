use ggez::*;
use nalgebra::Point2;

const MOVE_SPEED: f32 = 2.0;
const PLAYER_SIZE: f32 = 25.0;

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

struct Player {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            x: 30.0,
            y: 40.0,
            dx: 0.0,
            dy: 0.0,
        }
    }
}

struct Map {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Map {
    fn new() -> Self {
        Map {
            left: 0.0,
            right: 800.0,
            top: 0.0,
            bottom: 600.0,
        }
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

struct State {
    player: Player,
    map: Map,
    polygons: Vec<Polygon>,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(),
            map: Map::new(),
            polygons: vec![
                Polygon::new(vec![
                    Point2::new(250.0, 250.0),
                    Point2::new(300.0, 250.0),
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
            ],
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
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.player.x, self.player.y],
            PLAYER_SIZE,
            0.5,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        for polygon in &self.polygons {
            let mesh = graphics::Mesh::new_polygon(
                ctx,
                graphics::DrawMode::stroke(3.0),
                &polygon.verts,
                graphics::WHITE,
            )?;

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)
    }
}

fn tick(ctx: &mut Context, state: &mut State) {
    handle_keyboard_input(ctx, &mut state.player);

    handle_collisions(state);
}

fn handle_keyboard_input(ctx: &mut Context, player: &mut Player) {
    use ggez::input::keyboard::*;

    player.dx = 0.0;
    player.dy = 0.0;

    if is_key_pressed(ctx, KeyCode::W) {
        player.dy -= MOVE_SPEED;
    }

    if is_key_pressed(ctx, KeyCode::S) {
        player.dy += MOVE_SPEED;
    }

    if is_key_pressed(ctx, KeyCode::A) {
        player.dx -= MOVE_SPEED;
    }

    if is_key_pressed(ctx, KeyCode::D) {
        player.dx += MOVE_SPEED;
    }
}

fn handle_collisions(state: &mut State) {
    state.player.x = (state.player.x + state.player.dx)
        .max(state.map.left + PLAYER_SIZE)
        .min(state.map.right - PLAYER_SIZE);
    state.player.y = (state.player.y + state.player.dy)
        .max(state.map.top + PLAYER_SIZE)
        .min(state.map.bottom - PLAYER_SIZE);
}
