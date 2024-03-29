use std::marker::PhantomData;

use super::UiElement;
use ggez::{
    event::MouseButton,
    graphics::{Canvas, DrawParam, Drawable, PxScale, Rect, Text, TextFragment},
    Context, GameResult,
};
use nalgebra::Point2;

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
        let text_dim = text.dimensions(ctx).unwrap();
        let width_ratio = bounds.w / text_dim.w;
        let height_ratio = bounds.h / text_dim.h;

        let font_scale = if width_ratio < height_ratio {
            PxScale::from(text_dim.h * width_ratio)
        } else {
            PxScale::from(bounds.h)
        };

        // Set the text size
        text.set_scale(font_scale);

        Label {
            text,
            dest,
            phantom: PhantomData,
        }
    }
}

impl<T> UiElement<T> for Label<T> {
    fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        canvas.draw(&self.text, DrawParam::default().dest(self.dest));

        Ok(())
    }

    fn contains_point(&self, _ctx: &mut Context, _point: &Point2<f32>) -> bool {
        false
    }

    fn on_click(&self, _ctx: &mut Context, _button: MouseButton) -> Option<T> {
        None::<T>
    }
}
