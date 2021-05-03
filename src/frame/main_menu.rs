use super::{editor::EditorFrame, game::GameFrame, Frame, FrameEvent};
use crate::{
    gui::{button::Button, label::Label, UILayer},
    state::Input,
};
use ggez::{
    event::KeyCode,
    graphics::{self, Rect},
    Context, GameResult,
};

pub struct MainMenuFrame {
    ui_layer: UILayer,
}

impl MainMenuFrame {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut ui_layer = UILayer::new();

        let screen_coords = graphics::screen_coordinates(ctx);

        ui_layer.add(title_label(ctx, screen_coords)?);
        ui_layer.add(start_button(ctx, screen_coords)?);
        ui_layer.add(editor_button(ctx, screen_coords)?);
        ui_layer.add(quit_button(ctx, screen_coords)?);

        Ok(MainMenuFrame { ui_layer })
    }
}

impl Frame for MainMenuFrame {
    fn tick(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.ui_layer.draw(ctx)
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<FrameEvent> {
        let mut events = Vec::new();

        match input {
            Input::MouseDown { button, x, y } => {
                events.extend(self.ui_layer.mouse_press(ctx, button, x, y))
            }
            Input::KeyDown { key_code } => match key_code {
                KeyCode::Escape => events.push(FrameEvent::PopFrame),
                _ => {}
            },
            _ => {}
        };

        events
    }
}

fn title_label(ctx: &mut Context, screen_coords: Rect) -> GameResult<Label> {
    let bounds = Rect {
        x: screen_coords.x + 30.0,
        y: screen_coords.y + screen_coords.h / 4.0,
        w: screen_coords.w - 60.0,
        h: screen_coords.h / 4.0,
    };
    let label = Label::new(ctx, "Stealth Game!", bounds);
    Ok(label)
}

fn start_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
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
        Box::new(|| {
            let frame = Box::new(GameFrame::new("level1"));
            Some(FrameEvent::PushFrame(frame))
        }),
    )
}

fn editor_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
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
        Box::new(|| {
            let frame = Box::new(EditorFrame::new());
            Some(FrameEvent::PushFrame(frame))
        }),
    )
}

fn quit_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
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
        Box::new(|| Some(FrameEvent::PopFrame)),
    )
}
