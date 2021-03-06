pub mod game;
pub mod main_menu;

use crate::state::Input;
use ggez::{Context, GameResult};

pub enum FrameEvent {
    PopFrame,
    PushFrame(Box<dyn Frame>),
}

pub trait Frame {
    fn tick(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<FrameEvent>;
}
