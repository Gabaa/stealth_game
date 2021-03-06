use crate::game::Game;
use crate::{
    frame::{Frame, FrameEvent},
    state::MouseEvent,
};
use ggez::{Context, GameResult};

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

    fn mouse_update(&mut self, _ctx: &mut Context, _mouse_event: MouseEvent) -> Vec<FrameEvent> {
        vec![]
    }
}
