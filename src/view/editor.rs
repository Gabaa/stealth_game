use super::{View, ViewEvent};
use crate::{
    editor::SelectionHandler,
    game::{rendering::Renderer, Game},
    gui::{button::Button, UiLayer},
    state::Input,
};
use ggez::{
    event::{KeyCode, MouseButton},
    graphics::{self, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

pub const GRID_SIZE: f32 = 50.0;

fn snap_to_grid(point: Point2<f32>) -> Point2<f32> {
    let x = (point.x / GRID_SIZE).round() * GRID_SIZE;
    let y = (point.y / GRID_SIZE).round() * GRID_SIZE;
    Point2::new(x, y)
}

pub struct EditorView {
    game: Game,
    renderer: Renderer,
    ui: UiLayer,
    selection_handler: SelectionHandler,
    snap_to_grid: bool,
}

impl EditorView {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut ui = UiLayer::new();
        let screen_coords = graphics::screen_coordinates(ctx);

        let button_bounds = Rect::new(screen_coords.x + screen_coords.w - 160.0, 10.0, 150.0, 30.0);
        let button = Button::new(
            ctx,
            button_bounds,
            Some("Example button"),
            Box::new(|_| None),
        )?;

        ui.add(button);

        Ok(EditorView {
            game: Game::new(),
            renderer: Renderer::new(),
            ui,
            selection_handler: SelectionHandler::new(),
            snap_to_grid: false,
        })
    }
}

impl View for EditorView {
    fn tick(&mut self, _ctx: &mut Context) -> Vec<ViewEvent> {
        Vec::new()
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.renderer
            .render(ctx, &self.game, Some(&self.selection_handler))?;

        self.ui.draw(ctx)
    }

    fn receive_input(&mut self, _ctx: &mut Context, input: Input) -> Vec<ViewEvent> {
        let mut events = vec![];

        match input {
            Input::MouseDown { button, x, y } => {
                let pos = Point2::new(x, y);
                self.selection_handler
                    .handle_mouse_down(&mut self.game, button, pos)
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
            Input::MouseUp { button, .. } => self
                .selection_handler
                .handle_mouse_up(&mut self.game, button),
            Input::KeyDown { key_code } => match key_code {
                KeyCode::Escape => events.push(ViewEvent::PopView),
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
    use crate::view::editor::snap_to_grid;
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
