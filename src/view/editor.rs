use super::{View, ViewEvent};
use crate::{
    editor::{PolygonType, SelectionHandler, SelectionObject},
    game::{polygon::Polygon, rendering::Renderer, Game},
    gui::{button::Button, UiLayer},
    state::Input,
};
use ggez::{
    event::KeyCode,
    graphics::{self, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

pub const GRID_SIZE: f32 = 25.0;

fn snap_to_grid(point: Point2<f32>) -> Point2<f32> {
    let x = (point.x / GRID_SIZE).round() * GRID_SIZE;
    let y = (point.y / GRID_SIZE).round() * GRID_SIZE;
    Point2::new(x, y)
}

enum EditorEvent {
    ViewEvent(ViewEvent),
    CreateObstacle,
}

pub struct EditorView {
    game: Game,
    renderer: Renderer,
    ui: UiLayer<EditorEvent>,
    selection_handler: SelectionHandler,
    snap_to_grid: bool,
}

impl EditorView {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut ui = UiLayer::new();

        let screen_coords = graphics::screen_coordinates(ctx);
        let bounds = Rect::new(screen_coords.x + screen_coords.w - 160.0, 10.0, 150.0, 30.0);
        let on_click: Box<dyn Fn(&mut Context) -> Option<EditorEvent>> =
            Box::new(|_| Some(EditorEvent::CreateObstacle));
        let button = Button::new(ctx, bounds, Some("Create obstacle"), on_click)?;

        ui.add(button);

        Ok(EditorView {
            game: Game::new(),
            renderer: Renderer::new(),
            ui,
            selection_handler: SelectionHandler::new(),
            snap_to_grid: false,
        })
    }

    fn create_obstacle(&mut self) {
        let obstacle = Polygon::new(vec![
            Point2::new(100.0, 100.0),
            Point2::new(200.0, 100.0),
            Point2::new(200.0, 200.0),
            Point2::new(100.0, 200.0),
        ]);
        self.game.game_map.obstacles.push(obstacle);
    }

    fn delete_selected_object(&mut self) {
        match &self.selection_handler.selected_object {
            Some(obj) => match obj {
                SelectionObject::Actor { index } if *index != 0 => {
                    self.game.actors.remove(*index);
                    self.selection_handler.selected_object = None;
                }
                SelectionObject::Polygon {
                    polygon_type: PolygonType::Obstacle { index },
                } => {
                    self.game.game_map.obstacles.remove(*index);
                    self.selection_handler.selected_object = None;
                }
                _ => (),
            },
            None => (),
        }
    }

    fn handle_editor_events(&mut self, events: Vec<EditorEvent>) -> Vec<ViewEvent> {
        let mut view_events = Vec::new();

        for event in events {
            match event {
                EditorEvent::CreateObstacle => self.create_obstacle(),
                EditorEvent::ViewEvent(view_event) => view_events.push(view_event),
            }
        }

        view_events
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

    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<ViewEvent> {
        let mut events = vec![];

        match input {
            Input::MouseDown { button, x, y } => {
                let pos = Point2::new(x, y);
                self.selection_handler
                    .handle_mouse_down(&mut self.game, button, pos);

                events.extend(self.ui.mouse_press(ctx, button, x, y));
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
                KeyCode::Escape => events.push(EditorEvent::ViewEvent(ViewEvent::PopView)),
                KeyCode::LControl => self.snap_to_grid = true,
                KeyCode::O => self.create_obstacle(),
                KeyCode::Delete => self.delete_selected_object(),
                _ => {}
            },
            Input::KeyUp {
                key_code: KeyCode::LControl,
            } => self.snap_to_grid = false,
            _ => {}
        }

        self.handle_editor_events(events)
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
