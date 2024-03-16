use super::{editor::EditorView, levels::LevelsView, View, ViewEvent};
use crate::{
    gui::{button::Button, label::Label, UiLayer},
    state::Input,
};
use ggez::{
    graphics::{Canvas, Rect},
    input::keyboard::KeyCode,
    Context, GameResult,
};

pub struct MainMenuView {
    ui_layer: Option<UiLayer<ViewEvent>>,
}

impl MainMenuView {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(MainMenuView { ui_layer: None })
    }

    fn init_ui(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let mut ui_layer = UiLayer::new();

        let screen_coords = canvas.screen_coordinates().unwrap();

        ui_layer.add(title_label(ctx, screen_coords)?);
        ui_layer.add(start_button(ctx, screen_coords)?);
        ui_layer.add(editor_button(ctx, screen_coords)?);
        ui_layer.add(quit_button(ctx, screen_coords)?);

        self.ui_layer = Some(ui_layer);

        Ok(())
    }
}

impl View for MainMenuView {
    fn tick(&mut self, _ctx: &mut Context) -> Vec<ViewEvent> {
        Vec::new()
    }

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        if self.ui_layer.is_none() {
            self.init_ui(ctx, canvas)?;
        }

        self.ui_layer.as_ref().unwrap().draw(ctx, canvas)
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<ViewEvent> {
        let mut events = Vec::new();

        match &mut self.ui_layer {
            Some(ui_layer) => {
                match input {
                    Input::MouseDown { button, x, y } => {
                        events.extend(ui_layer.mouse_press(ctx, button, x, y))
                    }
                    Input::KeyDown {
                        key_code: KeyCode::Escape,
                    } => events.push(ViewEvent::PopView),
                    _ => {}
                };
            }
            None => {}
        }

        events
    }
}

fn title_label(ctx: &mut Context, screen_coords: Rect) -> GameResult<Label<ViewEvent>> {
    let bounds = Rect {
        x: screen_coords.x + 30.0,
        y: screen_coords.y + screen_coords.h / 4.0,
        w: screen_coords.w - 60.0,
        h: screen_coords.h / 4.0,
    };
    let label = Label::new(ctx, "Stealth Game!", bounds);
    Ok(label)
}

fn start_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button<ViewEvent>> {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 2.0,
        w: screen_coords.w / 2.0,
        h: 60.0,
    };

    Button::new(
        ctx,
        bounds,
        Some("Play"),
        Box::new(|ctx| {
            let view = Box::new(LevelsView::new(ctx).ok()?);
            Some(ViewEvent::PushView(view))
        }),
    )
}

fn editor_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button<ViewEvent>> {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 2.0 + 70.0,
        w: screen_coords.w / 2.0,
        h: 60.0,
    };

    Button::new(
        ctx,
        bounds,
        Some("Level editor"),
        Box::new(|ctx| {
            let view = EditorView::new(ctx).ok()?;
            Some(ViewEvent::PushView(Box::new(view)))
        }),
    )
}

fn quit_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button<ViewEvent>> {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + (screen_coords.h / 2.0) + 140.0,
        w: screen_coords.w / 2.0,
        h: 60.0,
    };

    Button::new(
        ctx,
        bounds,
        Some("Quit"),
        Box::new(|_| Some(ViewEvent::PopView)),
    )
}
