pub mod button;

use ggez::{nalgebra::Point2, Context, GameResult};

pub trait UIElement {
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn contains_point(&self, ctx: &mut Context, point: &Point2<f32>) -> bool;
    fn mouse_enter(&self, ctx: &mut Context);
    fn mouse_leave(&self, ctx: &mut Context);
    fn mouse_stay(&self, ctx: &mut Context);
    fn mouse_press(&self, ctx: &mut Context);
    fn mouse_release(&self, ctx: &mut Context);
}

pub struct UILayer {
    elements: Vec<Box<dyn UIElement>>,
    hovered_elements: Vec<usize>,
}

impl UILayer {
    pub fn new() -> Self {
        UILayer {
            elements: vec![],
            hovered_elements: vec![],
        }
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

    pub fn mouse_motion(&mut self, ctx: &mut Context, x: f32, y: f32) {
        for (i, element) in self.elements.iter().enumerate() {
            let point = Point2::new(x, y);
            let is_hovering = element.contains_point(ctx, &point);
            let hover_index = self.hovered_elements.iter().position(|x| x == &i);

            match (is_hovering, hover_index) {
                (true, Some(_)) => {
                    element.mouse_stay(ctx);
                }
                (true, None) => {
                    self.hovered_elements.push(i);
                    element.mouse_enter(ctx);
                }
                (false, Some(index)) => {
                    self.hovered_elements.remove(index);
                    element.mouse_leave(ctx);
                }
                _ => {}
            }
        }
    }
}
