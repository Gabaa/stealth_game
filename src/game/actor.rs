use {
    crate::{
        game::{controller::Controller, fov::FieldOfView, game_map::GameMap},
        nalgebra::{distance, Point2, Unit, Vector2},
    },
    ggez::{
        input::keyboard::{is_key_pressed, KeyCode},
        Context,
    },
};

pub struct Actor {
    pub pos: Point2<f32>,
    pub radius: f32,
    pub direction: Unit<Vector2<f32>>,
    pub fov: Box<dyn FieldOfView>,
    pub controller: Controller,
    pub move_speed: f32,
}

impl Actor {
    pub fn new(
        x: f32,
        y: f32,
        fov: Box<dyn FieldOfView>,
        controller: Controller,
        move_speed: f32,
    ) -> Self {
        Actor {
            pos: Point2::new(x, y),
            radius: 25.0,
            direction: Unit::new_normalize(Vector2::new(1.0, 0.0)),
            fov,
            controller,
            move_speed,
        }
    }

    pub fn is_player(&self) -> bool {
        match self.controller {
            Controller::Player() => true,
            _ => false,
        }
    }

    pub fn get_next_movement(&mut self, ctx: &Context) -> Vector2<f32> {
        match &mut self.controller {
            Controller::Player() => self.handle_player_movement(ctx),
            Controller::Guard(patrol) => {
                if distance(&self.pos, &patrol.points[patrol.i]) <= self.radius * 0.1 {
                    patrol.i = (patrol.i + 1) % patrol.points.len();
                }
                if distance(&self.pos, &patrol.points[patrol.i]) <= 2.0 {
                    patrol.points[patrol.i] - self.pos
                } else {
                    let direction = Unit::new_normalize(patrol.points[patrol.i] - self.pos);
                    direction.into_inner() * self.move_speed
                }
            }
        }
    }

    fn handle_player_movement(&self, ctx: &Context) -> Vector2<f32> {
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
        let move_speed = if is_sprinting(ctx) {
            2.0 * self.move_speed
        } else {
            self.move_speed
        };
        normalized_direction * move_speed
    }

    pub fn update_fov(&mut self, game_map: &GameMap) {
        self.fov.recalculate(self.pos, self.direction, game_map)
    }
}

fn is_sprinting(ctx: &Context) -> bool {
    is_key_pressed(ctx, KeyCode::LShift)
}
