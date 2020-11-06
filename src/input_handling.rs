use crate::nalgebra::Vector2;
use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::Context;

pub fn handle_keyboard_input(ctx: &mut Context) -> Vector2<f32> {
    handle_movement_keys(ctx)
}

fn handle_movement_keys(ctx: &Context) -> Vector2<f32> {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if is_key_pressed(ctx, KeyCode::W) {
        dy -= 1.0;
    }
    if is_key_pressed(ctx, KeyCode::S) {
        dy += 1.0;
    }
    if is_key_pressed(ctx, KeyCode::A) {
        dx -= 1.0;
    }
    if is_key_pressed(ctx, KeyCode::D) {
        dx += 1.0;
    }

    let direction = Vector2::new(dx, dy);
    if direction.x == 0.0 && direction.y == 0.0 {
        return direction;
    }
    let normalized_direction = direction.normalize();
    let move_speed = if is_sprinting(ctx) { 4.0 } else { 2.0 };
    normalized_direction * move_speed
}

fn is_sprinting(ctx: &Context) -> bool {
    is_key_pressed(ctx, KeyCode::LShift)
}
