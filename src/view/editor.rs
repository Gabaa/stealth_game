use super::{game::GameView, View, ViewEvent};
use crate::{
    editor::{PolygonType, SelectionHandler, SelectionObject},
    game::{actor::Actor, polygon::Polygon, rendering::Renderer, Game},
    gui::{
        button::{Button, ButtonClickHandler},
        UiLayer,
    },
    state::Input,
};
use ggez::{
    graphics::{Canvas, Rect},
    input::keyboard::KeyCode,
    Context, GameResult,
};
use nalgebra::Point2;
use std::{
    fs::File,
    path::{Path, PathBuf},
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
    CreateGuard,
    Preview,
    Save,
}

pub struct EditorView {
    game: Game,
    renderer: Renderer,
    ui: Option<UiLayer<EditorEvent>>,
    selection_handler: SelectionHandler,
    snap_to_grid: bool,
}

impl EditorView {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(EditorView {
            game: Game::new(),
            renderer: Renderer::new(),
            ui: None,
            selection_handler: SelectionHandler::new(),
            snap_to_grid: false,
        })
    }

    fn init_ui(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let mut ui = UiLayer::new();

        let screen_coords = canvas.screen_coordinates().unwrap();
        ui.add(Self::init_obstacle_button(ctx, screen_coords)?);
        ui.add(Self::init_guard_button(ctx, screen_coords)?);
        ui.add(Self::init_preview_button(ctx, screen_coords)?);
        ui.add(Self::init_save_button(ctx, screen_coords)?);

        self.ui = Some(ui);

        Ok(())
    }

    fn init_obstacle_button(
        ctx: &mut Context,
        screen_coords: Rect,
    ) -> GameResult<Button<EditorEvent>> {
        let bounds = Rect::new(
            screen_coords.x + screen_coords.w - 160.0,
            screen_coords.y + 10.0,
            150.0,
            30.0,
        );
        let on_click: Box<ButtonClickHandler<EditorEvent>> =
            Box::new(|_| Some(EditorEvent::CreateObstacle));
        Button::new(ctx, bounds, Some("Create obstacle"), on_click)
    }

    fn init_guard_button(
        ctx: &mut Context,
        screen_coords: Rect,
    ) -> GameResult<Button<EditorEvent>> {
        let bounds = Rect::new(
            screen_coords.x + screen_coords.w - 160.0,
            screen_coords.y + 50.0,
            150.0,
            30.0,
        );
        let on_click: Box<ButtonClickHandler<EditorEvent>> =
            Box::new(|_| Some(EditorEvent::CreateGuard));
        Button::new(ctx, bounds, Some("Create guard"), on_click)
    }

    fn init_preview_button(
        ctx: &mut Context,
        screen_coords: Rect,
    ) -> GameResult<Button<EditorEvent>> {
        let bounds = Rect::new(
            screen_coords.x + screen_coords.w - 160.0,
            screen_coords.y + screen_coords.h - 40.0,
            150.0,
            30.0,
        );
        let on_click: Box<ButtonClickHandler<EditorEvent>> =
            Box::new(|_| Some(EditorEvent::Preview));
        Button::new(ctx, bounds, Some("Preview"), on_click)
    }

    fn init_save_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button<EditorEvent>> {
        let bounds = Rect::new(
            screen_coords.x + screen_coords.w - 160.0,
            screen_coords.y + screen_coords.h - 80.0,
            150.0,
            30.0,
        );
        let on_click: Box<ButtonClickHandler<EditorEvent>> = Box::new(|_| Some(EditorEvent::Save));
        Button::new(ctx, bounds, Some("Save"), on_click)
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

    fn create_guard(&mut self) {
        let actor = Actor::new_guard(
            100.0,
            100.0,
            vec![
                Point2::new(50.0, 50.0),
                Point2::new(150.0, 50.0),
                Point2::new(150.0, 150.0),
                Point2::new(50.0, 150.0),
            ],
        );
        self.game.actors.push(actor);
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
                EditorEvent::CreateGuard => self.create_guard(),
                EditorEvent::ViewEvent(view_event) => view_events.push(view_event),
                EditorEvent::Preview => {
                    let level_info = self.game.to_level_info();
                    let view = Box::new(GameView::new(level_info));
                    let view_event = ViewEvent::PushView(view);
                    view_events.push(view_event)
                }
                EditorEvent::Save => {
                    // Make the LevelInfo
                    let level_info = self.game.to_level_info();

                    // Find a file name of the form level_x.json
                    let mut i = 1;
                    let mut path: PathBuf;
                    loop {
                        path = Path::new("levels").join(format!("level_{}", i));
                        path.set_extension("json");
                        if !path.exists() {
                            break;
                        }
                        i += 1
                    }

                    // Create the file and write the data
                    let file = File::create(path).expect("Error while creating level file");
                    serde_json::to_writer(file, &level_info).expect("Error while saving level");

                    // Exit the editor
                    view_events.push(ViewEvent::PopView);
                }
            }
        }

        view_events
    }
}

impl View for EditorView {
    fn tick(&mut self, _ctx: &mut Context) -> Vec<ViewEvent> {
        Vec::new()
    }

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        self.renderer
            .render(ctx, canvas, &self.game, Some(&self.selection_handler))?;

        if self.ui.is_none() {
            self.init_ui(ctx, canvas)?;
        }

        self.ui.as_ref().unwrap().draw(ctx, canvas)
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<ViewEvent> {
        let mut events = vec![];

        match &mut self.ui {
            Some(ui) => match input {
                Input::MouseDown { button, x, y } => {
                    let pos = Point2::new(x, y);
                    self.selection_handler
                        .handle_mouse_down(&mut self.game, button, pos);

                    events.extend(ui.mouse_press(ctx, button, x, y));
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
            },
            None => {}
        };

        self.handle_editor_events(events)
    }
}

#[cfg(test)]
mod tests {
    use crate::view::editor::snap_to_grid;
    use nalgebra::Point2;

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
