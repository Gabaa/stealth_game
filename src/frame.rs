use {
    crate::game::Game,
    ggez::{Context, GameResult},
};

pub trait Frame {
    fn tick(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
}

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

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.game.draw(ctx)
    }
}
