use crate::game::{
    actor::Actor, controller::Controller, game_map::GameMap, polygon::Polygon, Game,
};
use ggez::{
    event::MouseButton,
    nalgebra::{distance, Point2},
};
use std::cmp::Ordering::Equal;

#[derive(Debug, Clone, Copy)]
pub enum PolygonType {
    EndArea,
    Obstacle { index: usize },
}

impl PolygonType {
    fn find(self, game_map: &mut GameMap) -> &mut Polygon {
        match self {
            Self::EndArea => &mut game_map.end_area,
            Self::Obstacle { index } => game_map
                .obstacles
                .get_mut(index)
                .unwrap_or_else(|| panic!("Could not find obstacle at index {}", index)),
        }
    }
}

pub enum DragObject {
    Actor {
        index: usize,
    },
    GuardPathVertex {
        actor_index: usize,
        vertex_index: usize,
    },
    Polygon {
        polygon_type: PolygonType,
    },
    PolygonVertex {
        polygon_type: PolygonType,
        index: usize,
    },
}

#[derive(Debug)]
pub enum SelectionObject {
    Actor { index: usize },
    Polygon { polygon_type: PolygonType },
}

pub struct SelectionHandler {
    dragged_object: Option<DragObject>,
    pub selected_object: Option<SelectionObject>,
}

impl SelectionHandler {
    pub fn new() -> Self {
        SelectionHandler {
            dragged_object: None,
            selected_object: None,
        }
    }

    pub fn handle_mouse_down(
        &mut self,
        game: &mut Game,
        button: MouseButton,
        mouse_pos: Point2<f32>,
    ) {
        match button {
            MouseButton::Left => self.dragged_object = self.find_object_to_drag(game, mouse_pos),
            MouseButton::Right => self.delete_object(game, mouse_pos),
            _ => {}
        }
    }

    fn find_object_to_drag(&self, game: &mut Game, mouse_pos: Point2<f32>) -> Option<DragObject> {
        // Check if there is a draggable vertex under the mouse
        if let Some(SelectionObject::Polygon { polygon_type }) = self.selected_object {
            let polygon = polygon_type.find(&mut game.game_map);
            if let Some(i) = self.find_polygon_vertex_at(polygon, mouse_pos) {
                return Some(DragObject::PolygonVertex {
                    polygon_type,
                    index: i,
                });
            } else if let Some(i) = self.find_polygon_pseudo_vertex_at(polygon, mouse_pos) {
                // Add and drag
                polygon.verts.insert(i + 1, mouse_pos);
                return Some(DragObject::PolygonVertex {
                    polygon_type,
                    index: i + 1,
                });
            }
        } else if let Some(SelectionObject::Actor { index }) = self.selected_object {
            let actor = game.actors.get_mut(index);
            if let Some(Actor {
                controller: Controller::Guard(guard),
                ..
            }) = actor
            {
                if let Some(i) = self.find_polygon_vertex_at(&guard.points, mouse_pos) {
                    return Some(DragObject::GuardPathVertex {
                        actor_index: index,
                        vertex_index: i,
                    });
                } else if let Some(i) = self.find_polygon_pseudo_vertex_at(&guard.points, mouse_pos)
                {
                    guard.points.verts.insert(i + 1, mouse_pos);
                    return Some(DragObject::GuardPathVertex {
                        actor_index: index,
                        vertex_index: i + 1,
                    });
                }
            }
        }

        if let Some(i) = self.find_actor_at(game, mouse_pos) {
            return Some(DragObject::Actor { index: i });
        }

        if self.has_end_area_at(game, mouse_pos) {
            return Some(DragObject::Polygon {
                polygon_type: PolygonType::EndArea,
            });
        }

        if let Some(i) = self.find_polygon_at(game, mouse_pos) {
            return Some(DragObject::Polygon {
                polygon_type: PolygonType::Obstacle { index: i },
            });
        }

        None
    }

    fn delete_object(&mut self, game: &mut Game, mouse_pos: Point2<f32>) {
        if let Some(SelectionObject::Polygon { polygon_type }) = self.selected_object {
            let polygon = polygon_type.find(&mut game.game_map);

            if polygon.verts.len() > 3 {
                if let Some(i) = self.find_polygon_vertex_at(polygon, mouse_pos) {
                    polygon.verts.remove(i);
                    return;
                }
            }
        }
        if let Some(SelectionObject::Actor { index }) = self.selected_object {
            if let Some(actor) = game.actors.get_mut(index) {
                if let Controller::Guard(guard) = &mut actor.controller {
                    if guard.points.verts.len() > 3 {
                        if let Some(i) = self.find_polygon_vertex_at(&guard.points, mouse_pos) {
                            guard.points.verts.remove(i);
                            return;
                        }
                    }
                }
            }
        }
    }

    /// Return the index of the polygon vertex under the mouse if one exists, otherwise None
    fn find_polygon_vertex_at(&self, polygon: &Polygon, mouse_pos: Point2<f32>) -> Option<usize> {
        polygon
            .verts
            .iter()
            .position(|v| distance(v, &mouse_pos) <= 10.0)
    }

    /// Return the index of the polygon vertex under the mouse if one exists, otherwise None
    fn find_polygon_pseudo_vertex_at(
        &self,
        polygon: &Polygon,
        mouse_pos: Point2<f32>,
    ) -> Option<usize> {
        polygon
            .edges()
            .map(|(start, end)| {
                let avg_x = (start.x + end.x) / 2.0;
                let avg_y = (start.y + end.y) / 2.0;
                Point2::new(avg_x, avg_y)
            })
            .position(|v| distance(&v, &mouse_pos) <= 10.0)
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
        game.game_map.end_area.contains(mouse_pos)
    }

    fn find_polygon_at(&self, game: &mut Game, mouse_pos: Point2<f32>) -> Option<usize> {
        game.game_map
            .obstacles
            .iter()
            .enumerate()
            .find(|(_, obstacle)| obstacle.contains(mouse_pos))
            .map(|(i, _)| i)
    }

    pub fn handle_mouse_motion(&mut self, game: &mut Game, mouse_pos: Point2<f32>) {
        match &self.dragged_object {
            Some(object) => {
                match object {
                    DragObject::Actor { index } => {
                        let selected = game
                            .actors
                            .get_mut(*index)
                            .expect("could not find selected element");
                        selected.pos = mouse_pos;
                    }
                    &DragObject::GuardPathVertex {
                        actor_index,
                        vertex_index,
                    } => {
                        if let Some(actor) = game.actors.get_mut(actor_index) {
                            if let Controller::Guard(guard) = &mut actor.controller {
                                if let Some(vertex) = guard.points.verts.get_mut(vertex_index) {
                                    *vertex = mouse_pos
                                }
                            }
                        }
                    }
                    DragObject::Polygon { polygon_type } => {
                        let polygon = polygon_type.find(&mut game.game_map);

                        // Get centroid of polygon (avg of points)
                        let num_points = polygon.verts.len() as f32;
                        let (acc_x, acc_y) = polygon
                            .verts
                            .iter()
                            .map(|v| (v.x, v.y))
                            .reduce(|(x1, y1), (x2, y2)| (x1 + x2, y1 + y2))
                            .expect("no verts found");
                        let centroid = Point2::new(acc_x / num_points, acc_y / num_points);

                        let mouse_delta = mouse_pos - centroid;
                        for vertex in polygon_type.find(&mut game.game_map).verts.iter_mut() {
                            *vertex += mouse_delta;
                        }
                    }
                    DragObject::PolygonVertex {
                        polygon_type,
                        index,
                    } => {
                        let polygon = polygon_type.find(&mut game.game_map);
                        if let Some(vertex) = polygon.verts.get_mut(*index) {
                            *vertex = mouse_pos
                        }
                    }
                }
            }
            None => {}
        }
    }

    pub fn handle_mouse_up(&mut self, _game: &mut Game, button: MouseButton) {
        if button == MouseButton::Left {
            self.end_drag()
        }
    }

    fn end_drag(&mut self) {
        self.selected_object = match self.dragged_object {
            Some(DragObject::Actor { index }) => Some(SelectionObject::Actor { index }),
            Some(DragObject::GuardPathVertex { actor_index, .. }) => {
                Some(SelectionObject::Actor { index: actor_index })
            }
            Some(DragObject::Polygon { polygon_type }) => {
                Some(SelectionObject::Polygon { polygon_type })
            }
            Some(DragObject::PolygonVertex { polygon_type, .. }) => {
                Some(SelectionObject::Polygon { polygon_type })
            }
            None => None,
        };

        self.dragged_object = None;
    }
}
