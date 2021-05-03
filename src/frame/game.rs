use crate::game::{level_info::LevelInfo, renderer::Renderer, Game};
use crate::{
    frame::{Frame, FrameEvent},
    state::Input,
};
use ggez::{event::KeyCode, Context, GameResult};
use std::{fs::File, path::Path};

pub struct GameFrame {
    game: Game,
    renderer: Renderer,
}

impl GameFrame {
    pub fn new(level_name: &str) -> Self {
        let level_info = load_level_info(level_name);

        GameFrame {
            game: Game::from_level_info(level_info),
            renderer: Renderer::new(),
        }
    }
}

impl Frame for GameFrame {
    fn tick(&mut self, ctx: &mut Context) {
        self.game.tick(ctx);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.renderer.render(ctx, &self.game)
    }

    fn receive_input(&mut self, _ctx: &mut Context, input: Input) -> Vec<FrameEvent> {
        let mut events = Vec::new();

        match input {
            Input::KeyDown { key_code } => match key_code {
                KeyCode::Escape => events.push(FrameEvent::PopFrame),
                _ => {}
            },
            _ => {}
        };

        events
    }
}

fn load_level_info(level_name: &str) -> LevelInfo {
    let mut path = Path::new("levels").join(level_name);
    path.set_extension("json");

    match File::open(path) {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(e) => panic!("Could not read level file: {}", e),
    }
}
