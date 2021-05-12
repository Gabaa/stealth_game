use crate::game::Game;
use ggez::nalgebra::{distance, Point2, Vector2};
use std::cmp::Ordering::Equal;

pub enum DragObject {
    Actor { index: usize },
    EndArea,
}

#[derive(Debug)]
pub enum SelectionObject {
    Actor { index: usize },
    EndArea,
}

pub struct SelectionHandler {
    dragged_object: Option<DragObject>,
    drag_distance: f32,
    pub selected_object: Option<SelectionObject>,
}

impl SelectionHandler {
    pub fn new() -> Self {
        SelectionHandler {
            dragged_object: None,
            drag_distance: 0.0,
            selected_object: None,
        }
    }

    pub fn handle_mouse_down(&mut self, game: &mut Game, mouse_pos: Point2<f32>) {
        self.dragged_object = if let Some(i) = self.find_actor_at(game, mouse_pos) {
            Some(DragObject::Actor { index: i })
        } else if self.has_end_area_at(game, mouse_pos) {
            Some(DragObject::EndArea)
        } else {
            None
        }
    }

    /// Return the index of the actor under the mouse
    fn find_actor_at(&self, game: &mut Game, mouse_pos: Point2<f32>) -> Option<usize> {
        game.actors
            .iter()
            .enumerate()
            .map(|(i, actor)| (i, distance(&actor.pos, &mouse_pos) - actor.radius))
            .filter(|(_, dist)| *dist < 0.0)
            .min_by(|(_, dist_1), (_, dist_2)| dist_1.partial_cmp(&dist_2).unwrap_or(Equal))
            .map(|(i, _)| i)
    }

    fn has_end_area_at(&self, game: &mut Game, mouse_pos: Point2<f32>) -> bool {
        game.game_map.end_area.bounding_box().contains(mouse_pos)
    }

    pub fn handle_mouse_motion(&mut self, game: &mut Game, mouse_delta: Vector2<f32>) {
        match &self.dragged_object {
            Some(object) => {
                self.drag_distance += mouse_delta.norm();
                match object {
                    DragObject::Actor { index } => {
                        let selected = game
                            .actors
                            .iter_mut()
                            .nth(*index)
                            .expect("could not find selected element");
                        selected.pos += mouse_delta;
                    }
                    DragObject::EndArea => {
                        for vertex in &mut game.game_map.end_area.verts {
                            *vertex += mouse_delta;
                        }
                    }
                }
            }
            None => {}
        }
    }

    pub fn handle_mouse_up(&mut self, _game: &mut Game) {
        self.selected_object = if self.drag_distance < 5.0 {
            match self.dragged_object {
                Some(DragObject::Actor { index }) => Some(SelectionObject::Actor { index }),
                Some(DragObject::EndArea) => Some(SelectionObject::EndArea),
                None => None,
            }
        } else {
            None
        };

        self.dragged_object = None;
        self.drag_distance = 0.0;

        if self.selected_object.is_some() {
            println!("{:?}", self.selected_object.as_ref().unwrap())
        }
    }
}
