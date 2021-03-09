use {
    crate::game::{controller::Controller, fov::FieldOfView, game_map::GameMap},
    ggez::{
        nalgebra::{Point2, Unit, Vector2},
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
        matches!(self.controller, Controller::Player(_))
    }

    pub fn next_movement(&mut self, ctx: &Context) -> Vector2<f32> {
        self.controller
            .next_movement(ctx, self.pos, self.move_speed)
    }

    pub fn update_fov(&mut self, game_map: &GameMap) {
        self.fov.recalculate(self.pos, self.direction, game_map)
    }
}
