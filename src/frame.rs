use crate::{gui::UILayer, state::MouseEvent};

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
    fn mouse_update(&mut self, ctx: &mut Context, mouse_event: MouseEvent);
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

    fn mouse_update(&mut self, ctx: &mut Context, mouse_event: MouseEvent) {}
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

    fn mouse_update(&mut self, ctx: &mut Context, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::MOTION { x, y } => self.ui_layer.mouse_motion(ctx, x, y),
            MouseEvent::PRESS { button, x, y } => {}
            MouseEvent::RELEASE { button, x, y } => {}
        }

        // TODO: find en måde at lave en game frame
    }
}

fn start_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 2.0 - 50.0,
        w: screen_coords.w / 2.0,
        h: 100.0,
    };

    Button::new(ctx, bounds, Some("Play"))
}

fn quit_button(ctx: &mut Context, screen_coords: Rect) -> GameResult<Button> {
    let width = 150.0;
    let height = 50.0;

    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 2.0 - width / 2.0,
        y: screen_coords.y + screen_coords.h / 2.0,
        w: width,
        h: height,
    };

    Button::new(ctx, bounds, Some("Quit"))
}
