use crate::{
    actor::Actor,
    collision_handling::apply_physics_movement,
    controller::{Controller, Patrol},
    drawing::draw_all,
    fov::{ConeFieldOfView, NoFieldOfView},
    game_map::GameMap,
    nalgebra::Point2,
};
use ggez::{event, graphics, input::mouse::MouseButton, timer, Context, GameResult};

pub struct State {
    pub actors: Vec<Actor>,
    pub game_map: GameMap,
    pub player_won: bool,
    pub player_found: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            actors: vec![
                Actor::new(
                    30.0,
                    40.0,
                    Box::new(NoFieldOfView {}),
                    Controller::Player(),
                    2.0,
                ),
                Actor::new(
                    600.0,
                    50.0,
                    Box::new(ConeFieldOfView::new(90.0, 200.0)),
                    Controller::Guard(Patrol {
                        points: vec![
                            Point2::new(604.0, 96.0),
                            Point2::new(279.0, 72.0),
                            Point2::new(65.0, 345.0),
                            Point2::new(326.0, 511.0),
                            Point2::new(659.0, 357.0),
                        ],
                        i: 0,
                    }),
                    2.0,
                ),
            ],
            game_map: GameMap::new(),
            player_won: false,
            player_found: false,
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            tick(ctx, self);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        draw_all(ctx, &self)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        println!("{}, {}", x, y)
    }
}

fn tick(ctx: &mut Context, state: &mut State) {
    if state.player_won {
        println!("You won!");
        event::quit(ctx);
    }

    if state.player_found {
        println!("Player was discovered...");
        event::quit(ctx);
    }

    apply_physics_movement(state, ctx);

    for actor in &mut state.actors {
        actor.update_fov(&state.game_map);
    }
}
