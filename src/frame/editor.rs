use super::{Frame, FrameEvent};
use crate::{
    editor::SelectionHandler,
    game::{renderer::Renderer, Game},
    state::Input,
};
use ggez::{
    event::{KeyCode, MouseButton},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

pub struct EditorFrame {
    game: Game,
    renderer: Renderer,
    selection_handler: SelectionHandler,
}

impl EditorFrame {
    pub fn new() -> Self {
        EditorFrame {
            game: Game::new(),
            renderer: Renderer::new(),
            selection_handler: SelectionHandler::new(),
        }
    }
}

impl Frame for EditorFrame {
    fn tick(&mut self, _ctx: &mut Context) -> Vec<FrameEvent> {
        Vec::new()
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.renderer
            .render(ctx, &self.game, Some(&self.selection_handler))
    }

    fn receive_input(&mut self, _ctx: &mut Context, input: Input) -> Vec<FrameEvent> {
        let mut events = vec![];

        match input {
            Input::MouseDown { button, x, y } => {
                if let MouseButton::Left = button {
                    self.selection_handler
                        .handle_mouse_down(&mut self.game, Point2::new(x, y))
                }
            }
            Input::MouseMotion { dx, dy } => self
                .selection_handler
                .handle_mouse_motion(&mut self.game, Vector2::new(dx, dy)),
            Input::MouseUp { button, .. } => {
                if let MouseButton::Left = button {
                    self.selection_handler.handle_mouse_up(&mut self.game)
                }
            }
            Input::KeyDown { key_code } => match key_code {
                KeyCode::Escape => events.push(FrameEvent::PopFrame),
                _ => {}
            },
        }

        events
    }
}
