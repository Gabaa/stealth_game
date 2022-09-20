use super::{
    actor::Actor, controller::Controller, fov::FieldOfView, game_map::GameMap, polygon::Polygon,
    Game,
};
use crate::{
    editor::{PolygonType, SelectionHandler, SelectionObject},
    view::editor::GRID_SIZE,
};
use ggez::{
    graphics::{self, draw, Color, DrawMode, DrawParam, Mesh, Rect},
    Context, GameResult,
};
use nalgebra::Point2;

pub const GRID_LINE: Color = Color::new(0.3, 0.3, 0.3, 1.0);
pub const END_AREA: Color = Color::new(0.0, 1.0, 0.0, 0.1);
pub const END_AREA_SELECTED: Color = Color::new(0.5, 1.0, 0.5, 0.1);
pub const PLAYER_VISIBLE_AREA: Color = Color::new(1.0, 1.0, 1.0, 0.1);
pub const GUARD_VISIBLE_AREA: Color = Color::new(1.0, 0.0, 0.0, 0.1);
pub const GUARD: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub const GUARD_SELECTED: Color = Color::new(0.2, 0.2, 1.0, 1.0);
pub const OBSTACLE: Color = Color::new(0.4, 0.4, 0.4, 1.0);
pub const OBSTACLE_SELECTED: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(
        &self,
        ctx: &mut Context,
        game: &Game,
        selection_handler: Option<&SelectionHandler>,
    ) -> GameResult<()> {
        // TODO: These should re-use the meshes instead of remaking each time

        if selection_handler.is_some() {
            self.draw_grid(ctx)?;
        }
        self.draw_all_fov(ctx, &game.actors)?;
        self.draw_obstacles(ctx, &game.game_map, selection_handler)?;
        self.draw_end_area(ctx, &game.game_map, selection_handler)?;
        self.draw_actors(ctx, &game.actors, selection_handler)?;

        Ok(())
    }

    fn draw_grid(&self, ctx: &mut Context) -> GameResult {
        let screen_coords = graphics::screen_coordinates(ctx);

        let mut x = screen_coords.x;
        while x < screen_coords.x + screen_coords.w {
            let line = Mesh::new_line(
                ctx,
                &[
                    Point2::new(x, screen_coords.y),
                    Point2::new(x, screen_coords.y + screen_coords.h),
                ],
                1.0,
                GRID_LINE,
            )?;

            draw(ctx, &line, DrawParam::default())?;

            x += GRID_SIZE;
        }

        let mut y = screen_coords.y;
        while y < screen_coords.y + screen_coords.h {
            let line = Mesh::new_line(
                ctx,
                &[
                    Point2::new(screen_coords.x, y),
                    Point2::new(screen_coords.x + screen_coords.w, y),
                ],
                1.0,
                GRID_LINE,
            )?;

            draw(ctx, &line, DrawParam::default())?;

            y += GRID_SIZE;
        }

        Ok(())
    }

    fn draw_all_fov(&self, ctx: &mut Context, actors: &[Actor]) -> GameResult<()> {
        for actor in actors {
            let color = if actor.is_player() {
                PLAYER_VISIBLE_AREA
            } else {
                GUARD_VISIBLE_AREA
            };
            self.draw_fov(ctx, &*actor.fov, color)?;
        }

        Ok(())
    }

    fn draw_fov(&self, ctx: &mut Context, fov: &dyn FieldOfView, color: Color) -> GameResult<()> {
        let visible_area = match fov.get_visible_area() {
            Some(polygon) => polygon,
            None => return Ok(()),
        };

        if visible_area.verts.len() < 3 {
            return Ok(());
        }

        let mesh = Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &visible_area.verts, color)?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }

    fn draw_obstacles(
        &self,
        ctx: &mut Context,
        game_map: &GameMap,
        selection_handler: Option<&SelectionHandler>,
    ) -> GameResult<()> {
        for (i, polygon) in game_map.obstacles.iter().enumerate() {
            let is_selected = matches!(selection_handler,
                Some(&SelectionHandler {
                    selected_object:
                        Some(SelectionObject::Polygon {
                            polygon_type: PolygonType::Obstacle { index },
                        }),
                    ..
                }) if index == i,
            );

            let color = if is_selected {
                OBSTACLE_SELECTED
            } else {
                OBSTACLE
            };

            let mesh = Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &polygon.verts, color)?;

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

            if is_selected {
                self.draw_polygon_vertices(ctx, polygon)?;
            }
        }

        Ok(())
    }

    fn draw_end_area(
        &self,
        ctx: &mut Context,
        game_map: &GameMap,
        selection_handler: Option<&SelectionHandler>,
    ) -> GameResult<()> {
        let is_selected = matches!(
            selection_handler,
            Some(&SelectionHandler {
                selected_object: Some(SelectionObject::Polygon {
                    polygon_type: PolygonType::EndArea,
                }),
                ..
            })
        );

        let color = if is_selected {
            END_AREA_SELECTED
        } else {
            END_AREA
        };

        let mesh = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &game_map.end_area.verts,
            color,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Draw vertices if selected
        if is_selected {
            self.draw_polygon_vertices(ctx, &game_map.end_area)?;
        }

        Ok(())
    }

    fn draw_polygon_vertices(&self, ctx: &mut Context, polygon: &Polygon) -> GameResult {
        for vertex in &polygon.verts {
            self.draw_polygon_vertex(ctx, vertex, false)?;
        }

        // Draw pseudovertices on all edges
        for (start_point, end_point) in polygon.edges() {
            let avg_x = (start_point.x + end_point.x) / 2.0;
            let avg_y = (start_point.y + end_point.y) / 2.0;
            let middle_point = Point2::new(avg_x, avg_y);
            self.draw_polygon_vertex(ctx, &middle_point, true)?;
        }

        Ok(())
    }

    fn draw_polygon_vertex(
        &self,
        ctx: &mut Context,
        vertex: &Point2<f32>,
        pseudovertex: bool,
    ) -> GameResult {
        let color = if pseudovertex {
            Color::new(1.0, 1.0, 1.0, 0.2)
        } else {
            graphics::Color::WHITE
        };

        let mesh = Mesh::new_circle(ctx, graphics::DrawMode::fill(), *vertex, 5.0, 0.01, color)?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }

    fn draw_actors(
        &self,
        ctx: &mut Context,
        actors: &[Actor],
        selection_handler: Option<&SelectionHandler>,
    ) -> GameResult<()> {
        for (index, actor) in actors.iter().enumerate() {
            self.draw_actor(ctx, index, actor, selection_handler)?;
        }

        Ok(())
    }

    fn draw_actor(
        &self,
        ctx: &mut Context,
        index: usize,
        actor: &Actor,
        selection_handler: Option<&SelectionHandler>,
    ) -> GameResult<()> {
        let is_selected = matches!(
            selection_handler,
            Some(&SelectionHandler {
                selected_object: Some(SelectionObject::Actor { index: i }),
                ..
            }) if i == index
        );
        let mut color = graphics::Color::WHITE;

        if let Controller::Guard(guard) = &actor.controller {
            self.draw_discovery_bar(ctx, actor.discovered_player, &actor.pos, actor.radius)?;
            color = GUARD;

            if is_selected {
                self.draw_guard_patrol_path(ctx, &guard.points.verts)?;
                self.draw_polygon_vertices(ctx, &guard.points)?;
                color = GUARD_SELECTED;
            }
        }

        let mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [actor.pos.x, actor.pos.y],
            actor.radius,
            0.5,
            color,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }

    fn draw_discovery_bar(
        &self,
        ctx: &mut Context,
        discovered_player: f32,
        pos: &Point2<f32>,
        radius: f32,
    ) -> GameResult<()> {
        let height = radius / 2.0;
        let top = pos.y - radius * 1.5 - (height / 2.0);
        let width = radius * 2.0 * discovered_player;
        let left = pos.x - (width / 2.0);

        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(left, top, width, height),
            Color::from_rgb(100, 100, 255),
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }

    fn draw_guard_patrol_path(&self, ctx: &mut Context, points: &[Point2<f32>]) -> GameResult {
        let mesh = Mesh::new_polygon(ctx, DrawMode::stroke(2.0), points, graphics::Color::WHITE)?;
        draw(ctx, &mesh, DrawParam::default())
    }
}
