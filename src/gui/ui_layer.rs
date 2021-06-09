use ggez::{event::MouseButton, nalgebra::Point2, Context, GameResult};

pub trait UiElement<T> {
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn contains_point(&self, ctx: &mut Context, point: &Point2<f32>) -> bool;
    fn on_click(&self, ctx: &mut Context, button: MouseButton) -> Option<T>;
}

pub struct UiLayer<T> {
    elements: Vec<Box<dyn UiElement<T>>>,
}

impl<T> UiLayer<T> {
    pub fn new() -> Self {
        UiLayer { elements: vec![] }
    }

    pub fn add<E: 'static + UiElement<T>>(&mut self, element: E) {
        self.elements.push(Box::new(element));
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
    ) -> Vec<T> {
        let point = Point2::new(x, y);

        let mut events = Vec::new();
        for element in &self.elements {
            if element.contains_point(ctx, &point) {
                if let Some(e) = element.on_click(ctx, button) {
                    events.push(e)
                }
            }
        }
        events
    }
}
