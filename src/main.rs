use ggez::*;
use nalgebra::Point2;

const MOVE_SPEED: f32 = 2.0;

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
    radius: f32,
    dx: f32,
    dy: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            x: 30.0,
            y: 40.0,
            radius: 25.0,
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

struct GameMap {
    obstacles: Vec<Polygon>,
    end_area: Polygon,
}

impl GameMap {
    fn new() -> Self {
        GameMap {
            obstacles: vec![
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
            end_area: Polygon::new(vec![
                Point2::new(600.0, 400.0),
                Point2::new(800.0, 400.0),
                Point2::new(800.0, 600.0),
                Point2::new(600.0, 600.0),
            ]),
        }
    }
}

struct State {
    player: Player,
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
    let (dx, dy) = handle_obstacle_collisions(state);

    state.player.x += dx;
    state.player.y += dy;

    handle_end_area_intersection(state);
}

fn handle_obstacle_collisions(state: &mut State) -> (f32, f32) {
    let Player { x, y, .. } = state.player;
    let mut dx = state.player.dx;
    let mut dy = state.player.dy;

    for obstacle in &state.game_map.obstacles {
        let n = obstacle.verts.len();
        for i in 0..n {
            let a = obstacle.verts[i];
            let b = obstacle.verts[(i + 1) % n];
            let center = Point2::new(x + dx, y + dy);
            let closest_point = get_closest_point(a, b, center);

            let dist = nalgebra::distance(&closest_point, &center);
            if dist < state.player.radius {
                let direction = center - closest_point;
                let unit_direction = nalgebra::Unit::new_normalize(direction).into_inner();
                dx += unit_direction.x * (state.player.radius - dist) * 0.5;
                dy += unit_direction.y * (state.player.radius - dist) * 0.5;
            }
        }
    }

    (dx, dy)
}

fn handle_end_area_intersection(state: &mut State) {
    let end_area = &state.game_map.end_area;
    let position = Point2::new(state.player.x, state.player.y);

    let n = end_area.verts.len();
    for i in 0..n {
        let a = end_area.verts[i];
        let b = end_area.verts[(i + 1) % n];
        let closest_point = get_closest_point(a, b, position);

        let dist = nalgebra::distance(&closest_point, &position);
        if dist < state.player.radius {
            state.player_won = true;
        }
    }
}

fn get_closest_point(a: Point2<f32>, b: Point2<f32>, p: Point2<f32>) -> Point2<f32> {
    let ap = p - a;
    let ab = b - a;

    let ap_ab = ap.x * ab.x + ap.y * ab.y;
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let t = (ap_ab / ab2).max(0.0).min(1.0);

    a + ab * t
}
