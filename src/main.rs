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
    polygons: Vec<Polygon>,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(),
            polygons: vec![
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
    let mut new_player_x = state.player.x + state.player.dx;
    let mut new_player_y = state.player.y + state.player.dy;

    for polygon in &state.polygons {
        let n = polygon.verts.len();
        for i in 0..n {
            let new_center = Point2::new(new_player_x, new_player_y);
            
            let a = polygon.verts[i];
            let b = polygon.verts[(i + 1) % n];
            
            let closest_point = get_closest_point(a, b, new_center);

            let dist = nalgebra::distance(&closest_point, &new_center);
            if dist < PLAYER_SIZE {
                let direction = new_center - closest_point;
                let unit_direction = nalgebra::Unit::new_normalize(direction).into_inner();
                new_player_x += unit_direction.x * (PLAYER_SIZE - dist);
                new_player_y += unit_direction.y * (PLAYER_SIZE - dist);
            }
        }
    }

    state.player.x = new_player_x;
    state.player.y = new_player_y;
}

fn get_closest_point(a: Point2<f32>, b: Point2<f32>, p: Point2<f32>) -> Point2<f32> {
    let ap = p - a;
    let ab = b - a;

    let ap_ab = ap.x * ab.x + ap.y * ab.y;
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let t = (ap_ab / ab2).max(0.0).min(1.0);

    a + ab * t
}
