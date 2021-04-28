use ggez::{
    event::KeyCode,
    input::keyboard::is_key_pressed,
    nalgebra::{distance, Point2, Unit, Vector2},
    Context,
};

pub enum Controller {
    Player(PlayerController),
    Guard(GuardController),
}

impl Controller {
    pub fn new_player() -> Self {
        Controller::Player(PlayerController {})
    }

    pub fn new_guard(points: Vec<Point2<f32>>, i: usize) -> Self {
        Controller::Guard(GuardController { points, i })
    }

    pub fn next_movement(
        &mut self,
        ctx: &Context,
        pos: Point2<f32>,
        move_speed: f32,
    ) -> Vector2<f32> {
        match self {
            Controller::Player(player) => player.next_movement(ctx, move_speed),
            Controller::Guard(guard) => guard.next_movement(ctx, pos, move_speed),
        }
    }
}

pub struct PlayerController {}

impl PlayerController {
    fn next_movement(&mut self, ctx: &Context, move_speed: f32) -> Vector2<f32> {
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
        let move_speed = if is_key_pressed(ctx, KeyCode::LShift) {
            2.0 * move_speed
        } else {
            move_speed
        };
        normalized_direction * move_speed
    }
}

pub struct GuardController {
    pub points: Vec<Point2<f32>>,
    i: usize,
}

impl GuardController {
    fn next_movement(&mut self, _ctx: &Context, pos: Point2<f32>, move_speed: f32) -> Vector2<f32> {
        if distance(&pos, &self.points[self.i]) <= 5.0 {
            self.i = (self.i + 1) % self.points.len();
        }
        if distance(&pos, &self.points[self.i]) <= 2.0 {
            self.points[self.i] - pos
        } else {
            let direction = Unit::new_normalize(self.points[self.i] - pos);
            direction.into_inner() * move_speed
        }
    }
}
