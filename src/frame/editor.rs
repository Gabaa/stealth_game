use std::cmp::Ordering::Equal;

use super::{Frame, FrameEvent};
use crate::{
    editor::{DraggableObject, SelectionHandler},
    game::{renderer::Renderer, Game},
    state::Input,
};
use ggez::{
    event::MouseButton,
    nalgebra::{distance, Point2, Vector2},
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
        self.selection_handler.dragged_object = match self.selection_handler.dragged_object {
            Some(_) => None,
            None => {
                if let Some(i) = self.find_actor_at(mouse_pos) {
                    Some(DraggableObject::Actor { index: i })
                } else if self.has_end_area_at(mouse_pos) {
                    Some(DraggableObject::EndArea)
                } else {
                    None
                }
            }
        }
    }

    /// Return the index of the actor under the mouse
    fn find_actor_at(&self, mouse_pos: Point2<f32>) -> Option<usize> {
        self.game
            .actors
            .iter()
            .enumerate()
            .map(|(i, actor)| (i, distance(&actor.pos, &mouse_pos) - actor.radius))
            .filter(|(_, dist)| *dist < 0.0)
            .min_by(|(_, dist_1), (_, dist_2)| dist_1.partial_cmp(&dist_2).unwrap_or(Equal))
            .map(|(i, _)| i)
    }

    fn has_end_area_at(&self, mouse_pos: Point2<f32>) -> bool {
        self.game
            .game_map
            .end_area
            .bounding_box()
            .contains(mouse_pos)
    }

    fn handle_mouse_motion(&mut self, mouse_delta: Vector2<f32>) {
        match &self.selection_handler.dragged_object {
            Some(object) => match object {
                DraggableObject::Actor { index } => {
                    let selected = self
                        .game
                        .actors
                        .iter_mut()
                        .nth(*index)
                        .expect("could not find selected element");
                    selected.pos += mouse_delta;
                }
                DraggableObject::EndArea => {
                    for vertex in &mut self.game.game_map.end_area.verts {
                        *vertex += mouse_delta;
                    }
                }
            },

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
            Input::MouseMotion { dx, dy } => self.handle_mouse_motion(Vector2::new(dx, dy)),
            _ => {}
        }

        vec![]
    }
}
