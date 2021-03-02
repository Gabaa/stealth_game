use {
    crate::frame::{Frame, MainMenuFrame},
    ggez::{event, graphics, input::mouse::MouseButton, timer, Context, GameError, GameResult},
};

pub struct State {
    frame_stack: Vec<Box<dyn Frame>>,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(State {
            frame_stack: vec![Box::new(MainMenuFrame::new(ctx)?)],
        })
    }

    pub fn top_frame(&self) -> GameResult<&Box<dyn Frame>> {
        match (*self.frame_stack).last() {
            Some(frame) => Ok(frame),
            None => Err(GameError::WindowError("No frame".to_owned())),
        }
    }

    pub fn top_frame_mut(&mut self) -> GameResult<&mut Box<dyn Frame>> {
        match self.frame_stack.last_mut() {
            Some(frame) => Ok(frame),
            None => Err(GameError::WindowError("No frame".to_owned())),
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            self.top_frame_mut()?.tick(ctx)
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.top_frame()?.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        match self.top_frame_mut() {
            Ok(frame) => frame.mouse_update(ctx, MouseEvent::MOTION { x, y }),
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match self.top_frame_mut() {
            Ok(frame) => frame.mouse_update(ctx, MouseEvent::PRESS { button, x, y }),
            _ => {}
        }
        // self.frame_stack.push(Box::new(GameFrame::new()));
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match self.top_frame_mut() {
            Ok(frame) => frame.mouse_update(ctx, MouseEvent::RELEASE { button, x, y }),
            _ => {}
        }
    }
}

pub enum MouseEvent {
    MOTION { x: f32, y: f32 },
    PRESS { button: MouseButton, x: f32, y: f32 },
    RELEASE { button: MouseButton, x: f32, y: f32 },
}
