use super::fov::{ConeFieldOfView, NoFieldOfView};
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
        pos: Point2<f32>,
        fov: Box<dyn FieldOfView>,
        controller: Controller,
        move_speed: f32,
    ) -> Self {
        Actor {
            pos,
            radius: 25.0,
            direction: Unit::new_normalize(Vector2::new(1.0, 0.0)),
            fov,
            controller,
            move_speed,
        }
    }

    pub fn new_player(x: f32, y: f32) -> Self {
        Actor::new(
            Point2::new(x, y),
            Box::new(NoFieldOfView {}),
            Controller::new_player(),
            1.2,
        )
    }

    pub fn new_guard(x: f32, y: f32) -> Self {
        Actor::new(
            Point2::new(x, y),
            Box::new(ConeFieldOfView::new(90.0, 300.0)),
            Controller::new_guard(
                vec![
                    Point2::new(604.0, 96.0),
                    Point2::new(659.0, 357.0),
                    Point2::new(326.0, 511.0),
                    Point2::new(65.0, 345.0),
                    Point2::new(279.0, 72.0),
                ],
                0,
            ),
            1.3,
        )
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
