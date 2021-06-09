use std::marker::PhantomData;

use super::UiElement;
use ggez::{
    event::MouseButton,
    graphics::{draw, DrawParam, Font, Rect, Scale, Text, TextFragment},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Label<T> {
    text: Text,
    dest: Point2<f32>,
    phantom: PhantomData<T>,
}

impl<T> Label<T> {
    pub fn new(ctx: &mut Context, label_text: &str, bounds: Rect) -> Self {
        let dest = Point2::new(bounds.x, bounds.y);
        let mut text = Text::new(TextFragment::new(label_text));

        // Find out the maximal size of the text inside the bounds
        let (width, height) = text.dimensions(ctx);
        let width_ratio = bounds.w / width as f32;
        let height_ratio = bounds.h / height as f32;

        let font_scale = if width_ratio < height_ratio {
            Scale::uniform(height as f32 * width_ratio)
        } else {
            Scale::uniform(bounds.h)
        };

        // Set the text size
        text.set_font(Font::default(), font_scale);

        Label {
            text,
            dest,
            phantom: PhantomData,
        }
    }
}

impl<T> UiElement<T> for Label<T> {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        draw(ctx, &self.text, DrawParam::default().dest(self.dest))
    }

    fn contains_point(&self, _ctx: &mut Context, _point: &Point2<f32>) -> bool {
        false
    }

    fn on_click(&self, _ctx: &mut Context, _button: MouseButton) -> Option<T> {
        None::<T>
    }
}
