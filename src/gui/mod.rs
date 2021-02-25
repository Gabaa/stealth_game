pub mod button;

use ggez::{Context, GameResult};

pub trait UIElement {
    fn draw(&self, ctx: &mut Context) -> GameResult;
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
}
