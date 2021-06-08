pub mod editor;
pub mod game;
pub mod main_menu;

use crate::state::Input;
use ggez::{Context, GameResult};

pub enum ViewEvent {
    PopView,
    PushView(Box<dyn View>),
}

pub trait View {
    fn tick(&mut self, ctx: &mut Context) -> Vec<ViewEvent>;
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<ViewEvent>;
}
