use ggez::event::{KeyCode, KeyMods};

use crate::frame::{main_menu::MainMenuFrame, Frame, FrameEvent};
use ggez::{event, graphics, input::mouse::MouseButton, timer, Context, GameResult};

pub struct State {
    frame_stack: Vec<Box<dyn Frame>>,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(State {
            frame_stack: vec![Box::new(MainMenuFrame::new(ctx)?)],
        })
    }

    pub fn top_frame(&self) -> Option<&Box<dyn Frame>> {
        self.frame_stack.last()
    }

    pub fn top_frame_mut(&mut self) -> Option<&mut Box<dyn Frame>> {
        self.frame_stack.last_mut()
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) {
        if let Some(frame) = self.top_frame_mut() {
            let events = frame.receive_input(ctx, input);

            for event in events {
                match event {
                    FrameEvent::PopFrame => {
                        self.frame_stack.pop();
                    }
                    FrameEvent::PushFrame(frame) => self.frame_stack.push(frame),
                }

                if self.frame_stack.is_empty() {
                    event::quit(ctx);
                }
            }
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            if let Some(frame) = self.top_frame_mut() {
                frame.tick(ctx)
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if let Some(frame) = self.top_frame() {
            frame.draw(ctx)?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_code: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.receive_input(ctx, Input::KeyDown { key_code })
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.receive_input(ctx, Input::MouseDown { button, x, y })
    }
}

pub enum Input {
    MouseDown { button: MouseButton, x: f32, y: f32 },
    KeyDown { key_code: KeyCode },
}
