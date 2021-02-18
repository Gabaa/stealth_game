use {
    crate::frame::{Frame, GameFrame, MainMenuFrame},
    ggez::{event, graphics, input::mouse::MouseButton, timer, Context, GameError, GameResult},
};

pub struct State {
    frame_stack: Vec<Box<dyn Frame>>,
}

impl State {
    pub fn new() -> Self {
        State {
            frame_stack: vec![Box::new(MainMenuFrame {})],
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            match self.frame_stack.last_mut() {
                Some(frame) => (*frame).tick(ctx),
                None => return Err(GameError::EventLoopError("No frame".to_owned())),
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        match self.frame_stack.last() {
            Some(frame) => frame.draw(ctx),
            None => Err(GameError::EventLoopError("No frame".to_owned())),
        }?;

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.frame_stack.push(Box::new(GameFrame::new()));
        // println!("{}, {}", x, y)
    }
}
