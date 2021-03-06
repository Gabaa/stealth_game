use crate::game::Game;
use crate::{
    frame::{Frame, FrameEvent},
    state::Input,
};
use ggez::{event::KeyCode, Context, GameResult};

pub struct GameFrame {
    game: Game,
}

impl GameFrame {
    pub fn new() -> Self {
        GameFrame { game: Game::new() }
    }
}

impl Frame for GameFrame {
    fn tick(&mut self, ctx: &mut Context) {
        self.game.tick(ctx);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.game.draw(ctx)
    }

    fn receive_input(&mut self, _ctx: &mut Context, input: Input) -> Vec<FrameEvent> {
        let mut events = Vec::new();

        match input {
            Input::MouseDown { .. } => {}
            Input::KeyDown { key_code } => match key_code {
                KeyCode::Escape => events.push(FrameEvent::PopFrame),
                _ => {}
            },
        };

        events
    }
}
