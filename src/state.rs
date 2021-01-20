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
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player = Actor::new(
            Point2::new(30.0, 40.0),
            Box::new(NoFieldOfView {}),
            Controller::Player(),
            1.2,
            ctx,
        )?;

        let guard = Actor::new(
            Point2::new(600.0, 50.0),
            Box::new(ConeFieldOfView::new(90.0, 300.0)),
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
            1.3,
            ctx,
        )?;

        Ok(State {
            actors: vec![player, guard],
            game_map: GameMap::new(),
            player_won: false,
        })
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
    apply_physics_movement(state, ctx);

    if state.player_won {
        println!("You won!");
        event::quit(ctx);
    }

    if was_player_found(&state) {
        println!("Player was discovered...");
        event::quit(ctx);
    }

    for actor in &mut state.actors {
        actor.update_fov(&state.game_map);
    }
}

fn was_player_found(state: &State) -> bool {
    let mut player_pos_opt = None;

    for actor in &state.actors {
        if actor.is_player() {
            player_pos_opt = Some(actor.pos);
            break;
        }
    }

    match player_pos_opt {
        Some(player_pos) => {
            let mut found = false;

            for actor in &state.actors {
                if actor.is_player() {
                    continue;
                }
                found |= actor.fov.is_inside_fov(actor, &state.game_map, player_pos)
            }

            found
        }
        None => false,
    }
}
