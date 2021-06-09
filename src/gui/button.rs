use super::{label::Label, ui_layer::UiElement};
use ggez::{
    event::MouseButton,
    graphics::{draw, DrawMode, DrawParam, Drawable, Mesh, Rect, WHITE},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Button<T> {
    mesh: Mesh,
    label: Option<Label<T>>,
    handle_click: Box<dyn Fn(&mut Context) -> Option<T>>,
}

impl<T> Button<T> {
    pub fn new(
        ctx: &mut Context,
        bounds: Rect,
        button_text: Option<&str>,
        on_click: Box<dyn Fn(&mut Context) -> Option<T>>,
    ) -> GameResult<Self> {
        let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(3.0), bounds, WHITE)?;

        let label = match button_text {
            Some(text) => {
                let text_bounds = Rect::new(
                    bounds.x + 5.0,
                    bounds.y + 5.0,
                    bounds.w - 10.0,
                    bounds.h - 10.0,
                );
                Some(Label::new(ctx, text, text_bounds))
            }
            None => None,
        };

        Ok(Button {
            mesh,
            label,
            handle_click: on_click,
        })
    }
}

impl<T> UiElement<T> for Button<T> {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        draw(ctx, &self.mesh, DrawParam::new())?;

        match &self.label {
            Some(label) => label.draw(ctx),
            None => Ok(()),
        }
    }

    fn contains_point(&self, ctx: &mut Context, point: &Point2<f32>) -> bool {
        match self.mesh.dimensions(ctx) {
            Some(bounds) => bounds.contains(*point),
            _ => false,
        }
    }

    fn on_click(&self, ctx: &mut Context, button: MouseButton) -> Option<T> {
        match button {
            MouseButton::Left => (self.handle_click)(ctx),
            _ => None,
        }
    }
}
