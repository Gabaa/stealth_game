use {
    crate::{game::Game, gui::button::draw_button},
    ggez::{
        graphics::{self, Rect},
        Context, GameResult,
    },
};

pub trait Frame {
    fn tick(&mut self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
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

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.game.draw(ctx)
    }
}

pub struct MainMenuFrame {}

impl Frame for MainMenuFrame {
    fn tick(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let screen_coords = graphics::screen_coordinates(ctx);

        draw_start_game_button(ctx, screen_coords)?;
        draw_quit_button(ctx, screen_coords)?;

        Ok(())
    }
}

fn draw_start_game_button(ctx: &mut Context, screen_coords: Rect) -> GameResult {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 4.0,
        y: screen_coords.y + screen_coords.h / 2.0 - 50.0,
        w: screen_coords.w / 2.0,
        h: 100.0,
    };

    draw_button(ctx, bounds, "Play")?;

    Ok(())
}

fn draw_quit_button(ctx: &mut Context, screen_coords: Rect) -> GameResult {
    let bounds = Rect {
        x: screen_coords.x + screen_coords.w / 3.0,
        y: screen_coords.y + screen_coords.h / 2.0 + 150.0,
        w: screen_coords.w / 3.0,
        h: 50.0,
    };

    draw_button(ctx, bounds, "Quit")?;

    Ok(())
}
