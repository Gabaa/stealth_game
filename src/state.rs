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

    fn handle_event(&mut self, event: FrameEvent) {
        match event {
            FrameEvent::PopFrame => {
                self.frame_stack.pop();
            }
            FrameEvent::PushFrame(frame) => self.frame_stack.push(frame),
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
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if let Some(frame) = self.top_frame_mut() {
            let events = frame.mouse_update(ctx, MouseEvent::PRESS { button, x, y });
            for event in events {
                self.handle_event(event)
            }

            if self.frame_stack.is_empty() {
                event::quit(ctx);
            }
        }
    }
}

pub enum MouseEvent {
    PRESS { button: MouseButton, x: f32, y: f32 },
}
