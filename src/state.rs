use {
    crate::game::Game,
    ggez::{event, input::mouse::MouseButton, timer, Context, GameResult},
};

pub struct State {
    game: Game,
}

impl State {
    pub fn new() -> Self {
        State { game: Game::new() }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            self.game.tick(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.draw(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        println!("{}, {}", x, y)
    }
}
