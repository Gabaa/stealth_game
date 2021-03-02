use ggez::{
    graphics::{
        draw, Align, DrawMode, DrawParam, Drawable, Mesh, Rect, Scale, Text, TextFragment, WHITE,
    },
    nalgebra::Point2,
    Context, GameResult,
};

use super::UIElement;

pub struct Button {
    mesh: Mesh,
    text: Option<(Text, Point2<f32>)>,
}

impl Button {
    pub fn new(ctx: &mut Context, bounds: Rect, button_text: Option<&str>) -> GameResult<Self> {
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

        Ok(Button { mesh, text })
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

    fn mouse_enter(&self, _ctx: &mut Context) {
        match &self.text {
            Some((text, _)) => {
                println!("Entered {}", text.contents())
            }
            _ => {}
        }
    }

    fn mouse_leave(&self, _ctx: &mut Context) {}

    fn mouse_stay(&self, _ctx: &mut Context) {}

    fn mouse_press(&self, _ctx: &mut Context) {}

    fn mouse_release(&self, _ctx: &mut Context) {}
}
