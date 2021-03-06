use crate::{
    gui::UILayer,
    state::{FrameEvent, MouseEvent},
};

use {
    crate::{game::Game, gui::button::Button},
    ggez::{
        graphics::{self, Rect},
        Context, GameResult,
    },
};

pub trait Frame {
    fn tick(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
    fn mouse_update(&mut self, ctx: &mut Context, mouse_event: MouseEvent) -> Vec<FrameEvent>;
}

pub struct GameFrame {
    game: Game,
}

impl GameFrame {
    pub fn new() -> Self {
        GameFrame { game: Game::new() }
    }
}

impl Frame for GameFrame {
    fn tick(&mut self, ctx: &mut Context) {
        self.game.tick(ctx);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.game.draw(ctx)
    }

    fn mouse_update(&mut self, _ctx: &mut Context, _mouse_event: MouseEvent) -> Vec<FrameEvent> {
        vec![]
    }
}

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

    fn mouse_update(&mut self, ctx: &mut Context, mouse_event: MouseEvent) -> Vec<FrameEvent> {
        match mouse_event {
            MouseEvent::PRESS { button, x, y } => self.ui_layer.mouse_press(ctx, button, x, y),
        }
    }
}

fn start_button(ctx: &mut Context, _screen_coords: Rect) -> GameResult<Button> {
    /*
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 2.0 - 50.0,
        w: screen_coords.w / 2.0,
        h: 100.0,
    };
    */

    let bounds = Rect {
        x: 100.0,
        y: 100.0,
        w: 200.0,
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

fn quit_button(ctx: &mut Context, _screen_coords: Rect) -> GameResult<Button> {
    /*
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 2.0 - width / 2.0,
        y: screen_coords.y + screen_coords.h / 2.0,
        w: width,
        h: height,
    };
    */

    let bounds = Rect {
        x: 100.0,
        y: 250.0,
        w: 200.0,
        h: 100.0,
    };

    Button::new(
        ctx,
        bounds,
        Some("Quit"),
        Box::new(|| Some(FrameEvent::PopFrame)),
    )
}
