use ggez::{
    graphics::{draw, Align, DrawMode, DrawParam, Mesh, Rect, Scale, Text, TextFragment, WHITE},
    nalgebra::Point2,
    Context, GameResult,
};

pub fn draw_button(ctx: &mut Context, bounds: Rect, text: &str) -> GameResult {
    let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(3.0), bounds, WHITE)?;
    draw(ctx, &mesh, DrawParam::new())?;

    let mut text = Text::new(TextFragment::new(text).scale(Scale::uniform(bounds.h * 0.9)));
    text.set_bounds(Point2::new(bounds.w, f32::INFINITY), Align::Center);
    let dest = Point2::new(bounds.x, bounds.y);
    draw(ctx, &text, DrawParam::default().dest(dest))
}
