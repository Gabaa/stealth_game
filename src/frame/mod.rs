pub mod game;
pub mod main_menu;

use crate::state::MouseEvent;
use ggez::{Context, GameResult};

pub enum FrameEvent {
    PopFrame,
    PushFrame(Box<dyn Frame>),
}

pub trait Frame {
    fn tick(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
    fn mouse_update(&mut self, ctx: &mut Context, mouse_event: MouseEvent) -> Vec<FrameEvent>;
}
