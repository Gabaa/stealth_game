use super::{game::GameFrame, Frame, FrameEvent};
use crate::{
    gui::{button::Button, UILayer},
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

        let start_button = start_button(ctx, screen_coords)?;
        ui_layer.add(Box::new(start_button));

        let quit_button = quit_button(ctx, screen_coords)?;
        ui_layer.add(Box::new(quit_button));

        Ok(MainMenuFrame { ui_layer })
    }
}

impl Frame for MainMenuFrame {
    fn tick(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.ui_layer.draw(ctx)
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) -> Vec<FrameEvent> {
        match input {
            Input::MouseDown { button, x, y } => self.ui_layer.mouse_press(ctx, button, x, y),
            Input::KeyDown { key_code } => {
                let mut events = Vec::new();

                match key_code {
                    KeyCode::Escape => events.push(FrameEvent::PopFrame),
                    _ => {}
                };

                events
            }
        }
    }
}

fn start_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 4.0,
        w: screen_coords.w / 2.0,
        h: 100.0,
    };

    Button::new(
        ctx,
        bounds,
        Some("Play"),
        Box::new(|| {
            let frame = Box::new(GameFrame::new());
            Some(FrameEvent::PushFrame(frame))
        }),
    )
}

fn quit_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 2.0,
        w: screen_coords.w / 2.0,
        h: 100.0,
    };

    Button::new(
        ctx,
        bounds,
        Some("Quit"),
        Box::new(|| Some(FrameEvent::PopFrame)),
    )
}
