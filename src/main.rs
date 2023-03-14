#![windows_subsystem = "windows"]

mod editor;
mod game;
mod gui;
mod state;
mod view;

use ggez::GameResult;
use ggez::{conf, event, ContextBuilder};
use state::State;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("stealth_game", "Gabaa")
        .window_setup(conf::WindowSetup::default().title("Stealth Game!!!"))
        .build()?;

    let state = State::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
