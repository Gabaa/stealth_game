use crate::player::Player;
use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::Context;

pub fn handle_keyboard_input(ctx: &mut Context, player: &mut Player) {
    handle_sprint_key(ctx, player);
    let (dx, dy) = handle_direction_keys(ctx);
    player.set_direction(dx, dy)
}

fn handle_direction_keys(ctx: &Context) -> (f32, f32) {
    let dx = 0.0;
    let dy = 0.0;

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

    (dx, dy)
}

fn handle_sprint_key(ctx: &Context, player: &mut Player) {
    if is_key_pressed(ctx, KeyCode::LShift) {
        player.set_speed(4.0);
    } else {
        player.set_speed(2.0);
    }
}
