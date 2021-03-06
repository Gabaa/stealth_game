pub mod button;

use ggez::{event::MouseButton, nalgebra::Point2, Context, GameResult};

use crate::state::FrameEvent;

pub trait UIElement {
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn contains_point(&self, ctx: &mut Context, point: &Point2<f32>) -> bool;
    fn on_click(&self, ctx: &mut Context, button: MouseButton) -> Option<FrameEvent>;
}

pub struct UILayer {
    elements: Vec<Box<dyn UIElement>>,
}

impl UILayer {
    pub fn new() -> Self {
        UILayer { elements: vec![] }
    }

    pub fn add(&mut self, element: Box<dyn UIElement>) {
        self.elements.push(element);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for element in &self.elements {
            element.draw(ctx)?;
        }

        Ok(())
    }

    pub fn mouse_press(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Vec<FrameEvent> {
        let point = Point2::new(x, y);

        let mut events = Vec::new();
        for element in &self.elements {
            if element.contains_point(ctx, &point) {
                match element.on_click(ctx, button) {
                    Some(e) => events.push(e),
                    _ => {}
                }
            }
        }
        events
    }
}
