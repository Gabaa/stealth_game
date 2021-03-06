use ggez::{
    event::MouseButton,
    graphics::{
        draw, Align, DrawMode, DrawParam, Drawable, Mesh, Rect, Scale, Text, TextFragment, WHITE,
    },
    nalgebra::Point2,
    Context, GameResult,
};

use crate::state::FrameEvent;

use super::UIElement;

pub struct Button {
    mesh: Mesh,
    text: Option<(Text, Point2<f32>)>,
    handle_click: Box<dyn Fn() -> Option<FrameEvent>>,
}

impl Button {
    pub fn new(
        ctx: &mut Context,
        bounds: Rect,
        button_text: Option<&str>,
        on_click: Box<dyn Fn() -> Option<FrameEvent>>,
    ) -> GameResult<Self> {
        let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(3.0), bounds, WHITE)?;

        let text = match button_text {
            Some(text) => {
                let mut text =
                    Text::new(TextFragment::new(text).scale(Scale::uniform(bounds.h * 0.9)));
                text.set_bounds(Point2::new(bounds.w, f32::INFINITY), Align::Center);
                let dest = Point2::new(bounds.x, bounds.y);
                Some((text, dest))
            }
            None => None,
        };

        Ok(Button {
            mesh,
            text,
            handle_click: on_click,
        })
    }
}

impl UIElement for Button {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        draw(ctx, &self.mesh, DrawParam::new())?;

        match &self.text {
            Some((text, dest)) => draw(ctx, text, DrawParam::default().dest(*dest)),
            None => Ok(()),
        }
    }

    fn contains_point(&self, ctx: &mut Context, point: &Point2<f32>) -> bool {
        match self.mesh.dimensions(ctx) {
            Some(bounds) => bounds.contains(*point),
            _ => false,
        }
    }

    fn on_click(&self, _ctx: &mut Context, button: MouseButton) -> Option<FrameEvent> {
        match button {
            MouseButton::Left => (self.handle_click)(),
            _ => None,
        }
    }
}
