use super::{
    game::{load_level_info, GameView},
    View, ViewEvent,
};
use crate::{
    gui::{button::Button, UiLayer},
    state::Input,
};
use ggez::{
    event::KeyCode,
    graphics::{self, Rect},
    Context, GameResult,
};
use std::fs;
use std::path::Path;

pub struct LevelsView {
    ui_layer: UiLayer<ViewEvent>,
}

impl LevelsView {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut ui_layer = UiLayer::new();

        let screen_coords = graphics::screen_coordinates(ctx);
        let level_names = get_all_level_names();

        let mut y = 30.0;

        for level_name in level_names {
            let bounds = Rect::new(
                screen_coords.x + screen_coords.w / 4.0,
                screen_coords.y + y,
                screen_coords.w / 2.0,
                60.0,
            );
            ui_layer.add(level_button(ctx, bounds, level_name)?);
            y += 70.0;
        }

        Ok(LevelsView { ui_layer })
    }
}

impl View for LevelsView {
    fn tick(&mut self, _ctx: &mut Context) -> Vec<ViewEvent> {
        Vec::new()
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.ui_layer.draw(ctx)
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<ViewEvent> {
        let mut events = Vec::new();

        match input {
            Input::MouseDown { button, x, y } => {
                events.extend(self.ui_layer.mouse_press(ctx, button, x, y))
            }
            Input::KeyDown {
                key_code: KeyCode::Escape,
            } => events.push(ViewEvent::PopView),
            _ => {}
        };

        events
    }
}

fn get_all_level_names() -> Vec<String> {
    let levels_directory = Path::new("levels");
    fs::read_dir(levels_directory)
        .expect("Could not read levels")
        .into_iter()
        .map(|level| {
            level
                .expect("Failed to read level")
                .file_name()
                .into_string()
                .expect("Could not read level name")
        })
        .map(|name| name.replace(".json", ""))
        .collect()
}

fn level_button(
    ctx: &mut Context,
    bounds: Rect,
    level_name: String,
) -> GameResult<Button<ViewEvent>> {
    let display_name = level_display_name(&level_name);

    Button::new(
        ctx,
        bounds,
        Some(&display_name),
        Box::new(move |_| {
            let level_info = load_level_info(&level_name);
            let view = Box::new(GameView::new(level_info));
            Some(ViewEvent::PushView(view))
        }),
    )
}

fn level_display_name(level_name: &str) -> String {
    let display_name = level_name.replace('_', " ");
    let mut display_name_chars = display_name.chars();
    match display_name_chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + display_name_chars.as_str(),
    }
}
