use super::{Frame, FrameEvent};
use crate::{
    editor::SelectionHandler,
    game::{rendering::Renderer, Game},
    state::Input,
};
use ggez::{
    event::{KeyCode, MouseButton},
    nalgebra::Point2,
    Context, GameResult,
};

const GRID_SIZE: f32 = 50.0;

fn snap_to_grid(point: Point2<f32>) -> Point2<f32> {
    let x = (point.x / GRID_SIZE).round() * GRID_SIZE;
    let y = (point.y / GRID_SIZE).round() * GRID_SIZE;
    Point2::new(x, y)
}

pub struct EditorFrame {
    game: Game,
    renderer: Renderer,
    selection_handler: SelectionHandler,
    snap_to_grid: bool,
}

impl EditorFrame {
    pub fn new() -> Self {
        EditorFrame {
            game: Game::new(),
            renderer: Renderer::new(),
            selection_handler: SelectionHandler::new(),
            snap_to_grid: false,
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
            Input::MouseDown {
                button: MouseButton::Left,
                x,
                y,
            } => {
                let pos = Point2::new(x, y);
                self.selection_handler
                    .handle_mouse_down(&mut self.game, pos)
            }
            Input::MouseMotion { x, y } => {
                let mouse_pos = Point2::new(x, y);

                self.selection_handler.handle_mouse_motion(
                    &mut self.game,
                    if self.snap_to_grid {
                        snap_to_grid(mouse_pos)
                    } else {
                        mouse_pos
                    },
                );
            }
            Input::MouseUp {
                button: MouseButton::Left,
                ..
            } => self.selection_handler.handle_mouse_up(&mut self.game),
            Input::KeyDown { key_code } => match key_code {
                KeyCode::Escape => events.push(FrameEvent::PopFrame),
                KeyCode::LControl => self.snap_to_grid = true,
                _ => {}
            },
            Input::KeyUp {
                key_code: KeyCode::LControl,
            } => self.snap_to_grid = false,
            _ => {}
        }

        events
    }
}

#[cfg(test)]
mod tests {
    use crate::frame::editor::snap_to_grid;
    use ggez::nalgebra::Point2;

    #[test]
    fn snap_down() {
        let point = Point2::new(50.0, 60.0);
        let snapped_point = snap_to_grid(point);
        let expected = Point2::new(50.0, 50.0);
        assert_eq!(expected, snapped_point)
    }

    #[test]
    fn snap_up() {
        let point = Point2::new(50.0, 40.0);
        let snapped_point = snap_to_grid(point);
        let expected = Point2::new(50.0, 50.0);
        assert_eq!(expected, snapped_point)
    }
}
