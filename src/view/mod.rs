pub mod editor;
pub mod game;
pub mod levels;
pub mod main_menu;

use crate::state::Input;
use ggez::{graphics::Canvas, Context, GameResult};

pub enum ViewEvent {
    PopView,
    PushView(Box<dyn View>),
}

pub trait View {
    fn tick(&mut self, ctx: &mut Context) -> Vec<ViewEvent>;
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<ViewEvent>;
}
