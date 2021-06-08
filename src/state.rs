use ggez::event::{KeyCode, KeyMods};

use crate::view::{main_menu::MainMenuView, View, ViewEvent};
use ggez::{event, graphics, input::mouse::MouseButton, timer, Context, GameResult};

pub struct State {
    view_stack: Vec<Box<dyn View>>,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(State {
            view_stack: vec![Box::new(MainMenuView::new(ctx)?)],
        })
    }

    fn top_view(&self) -> Option<&Box<dyn View>> {
        self.view_stack.last()
    }

    fn top_view_mut(&mut self) -> Option<&mut Box<dyn View>> {
        self.view_stack.last_mut()
    }

    fn receive_input(&mut self, ctx: &mut Context, input: Input) {
        if let Some(view) = self.top_view_mut() {
            let events = view.receive_input(ctx, input);
            self.handle_events(ctx, events)
        }
    }

    fn handle_events(&mut self, ctx: &mut Context, events: Vec<ViewEvent>) {
        for event in events {
            match event {
                ViewEvent::PopView => {
                    self.view_stack.pop();
                }
                ViewEvent::PushView(view) => self.view_stack.push(view),
            }

            if self.view_stack.is_empty() {
                event::quit(ctx);
            }
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            if let Some(view) = self.top_view_mut() {
                let events = view.tick(ctx);
                self.handle_events(ctx, events);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if let Some(view) = self.top_view() {
            view.draw(ctx)?;
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

    fn key_up_event(&mut self, ctx: &mut Context, key_code: KeyCode, _keymods: KeyMods) {
        self.receive_input(ctx, Input::KeyUp { key_code })
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.receive_input(ctx, Input::MouseDown { button, x, y })
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.receive_input(ctx, Input::MouseMotion { x, y })
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.receive_input(ctx, Input::MouseUp { button, x, y });
    }
}

pub enum Input {
    MouseDown { button: MouseButton, x: f32, y: f32 },
    MouseMotion { x: f32, y: f32 },
    MouseUp { button: MouseButton, x: f32, y: f32 },
    KeyDown { key_code: KeyCode },
    KeyUp { key_code: KeyCode },
}
