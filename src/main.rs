use ggez::*;

const MOVE_SPEED: f32 = 1.0;
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

struct State {
    player_x: f32,
    player_y: f32,
}

impl State {
    fn new() -> Self {
        State {
            player_x: 30.0,
            player_y: 40.0,
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            handle_keyboard_input(self, ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // TODO: don't make a new mesh every draw call, just update an existing one
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.player_x, self.player_y],
            PLAYER_SIZE,
            0.5,
            graphics::WHITE,
        )
        .unwrap();

        match graphics::draw(ctx, &mesh, graphics::DrawParam::default()) {
            Ok(_) => {}
            Err(_) => {}
        };

        graphics::present(ctx)
    }
}

fn handle_keyboard_input(state: &mut State, ctx: &mut Context) {
    use ggez::input::keyboard::*;

    if is_key_pressed(ctx, KeyCode::W) {
        state.player_y -= MOVE_SPEED;
    }

    if is_key_pressed(ctx, KeyCode::S) {
        state.player_y += MOVE_SPEED;
    }

    if is_key_pressed(ctx, KeyCode::A) {
        state.player_x -= MOVE_SPEED;
    }

    if is_key_pressed(ctx, KeyCode::D) {
        state.player_x += MOVE_SPEED;
    }
}
