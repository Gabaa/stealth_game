use std::cmp::Ordering::Equal;

use super::{Frame, FrameEvent};
use crate::{
    editor::SelectionHandler,
    game::{renderer::Renderer, Game},
    state::Input,
};
use ggez::{
    event::MouseButton,
    nalgebra::{distance, Point2},
    Context, GameResult,
};

pub struct EditorFrame {
    game: Game,
    renderer: Renderer,
    selection_handler: SelectionHandler,
}

impl EditorFrame {
    pub fn new() -> Self {
        EditorFrame {
            game: Game::new(),
            renderer: Renderer::new(),
            selection_handler: SelectionHandler::new(),
        }
    }

    fn handle_mouse_down(&mut self, mouse_pos: Point2<f32>) {
        self.selection_handler.dragged_actor = match self.selection_handler.dragged_actor {
            Some(_) => None,
            None => {
                let closest = self
                    .game
                    .actors
                    .iter()
                    .map(|a| (a, distance(&a.pos, &mouse_pos)))
                    .enumerate()
                    .min_by(|(_, (_, dist_a)), (_, (_, dist_b))| {
                        dist_a.partial_cmp(dist_b).unwrap_or(Equal)
                    });

                match closest {
                    Some((i, (actor, distance))) => {
                        if distance < actor.radius {
                            Some(i)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    fn handle_mouse_motion(&mut self, mouse_pos: Point2<f32>) {
        match self.selection_handler.dragged_actor {
            Some(i) => {
                let selected = self
                    .game
                    .actors
                    .iter_mut()
                    .nth(i)
                    .expect("could not find selected element");
                selected.pos = mouse_pos;
            }
            None => {}
        }
    }
}

impl<'a> Frame for EditorFrame {
    fn tick(&mut self, _ctx: &mut Context) {}

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.renderer.render(ctx, &self.game)
    }

    fn receive_input(&mut self, _ctx: &mut Context, input: Input) -> Vec<FrameEvent> {
        match input {
            Input::MouseDown { button, x, y } => {
                if let MouseButton::Left = button {
                    self.handle_mouse_down(Point2::new(x, y))
                }
            }
            Input::MouseMotion { x, y } => self.handle_mouse_motion(Point2::new(x, y)),
            _ => {}
        }

        vec![]
    }
}
