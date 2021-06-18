use crate::game::{level_info::LevelInfo, rendering::Renderer, Game};
use crate::{
    state::Input,
    view::{View, ViewEvent},
};
use ggez::{event::KeyCode, Context, GameResult};
use std::{fs::File, path::Path};

pub struct GameView {
    game: Game,
    renderer: Renderer,
}

impl GameView {
    pub fn new(level_info: LevelInfo) -> Self {
        GameView {
            game: Game::from_level_info(level_info),
            renderer: Renderer::new(),
        }
    }
}

impl View for GameView {
    fn tick(&mut self, ctx: &mut Context) -> Vec<ViewEvent> {
        self.game.tick(ctx)
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.renderer.render(ctx, &self.game, None)
    }

    fn receive_input(&mut self, _ctx: &mut Context, input: Input) -> Vec<ViewEvent> {
        let mut events = Vec::new();

        if let Input::KeyDown {
            key_code: KeyCode::Escape,
        } = input
        {
            events.push(ViewEvent::PopView)
        };

        events
    }
}

pub fn load_level_info(level_name: &str) -> LevelInfo {
    let mut path = Path::new("levels").join(level_name);
    path.set_extension("json");

    match File::open(path) {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(e) => panic!("Could not read level file: {}", e),
    }
}
