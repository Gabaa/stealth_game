use crate::editor::{PolygonType, SelectionHandler, SelectionObject};

use super::{actor::Actor, fov::FieldOfView, game_map::GameMap, Game};
use ggez::{
    graphics::{self, Color, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

pub const END_AREA: Color = Color::new(0.0, 1.0, 0.0, 0.1);
pub const END_AREA_SELECTED: Color = Color::new(0.5, 1.0, 0.5, 0.1);
pub const PLAYER_VISIBLE_AREA: Color = Color::new(1.0, 1.0, 1.0, 0.1);
pub const GUARD_VISIBLE_AREA: Color = Color::new(1.0, 0.0, 0.0, 0.1);
pub const OBSTACLE: Color = Color::new(0.4, 0.4, 0.4, 1.0);

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
        self.draw_all_fov(ctx, &game.actors)?;
        self.draw_obstacles(ctx, &game.game_map)?;
        self.draw_end_area(ctx, &game.game_map, selection_handler)?;
        self.draw_actors(ctx, &game.actors)
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

    fn draw_fov(
        &self,
        ctx: &mut Context,
        fov: &dyn FieldOfView,
        color: graphics::Color,
    ) -> GameResult<()> {
        let visible_area = match fov.get_visible_area() {
            Some(polygon) => polygon,
            None => return Ok(()),
        };

        if visible_area.verts.len() < 3 {
            return Ok(());
        }

        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &visible_area.verts,
            color,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }

    fn draw_obstacles(&self, ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
        for polygon in &game_map.obstacles {
            let mesh = graphics::Mesh::new_polygon(
                ctx,
                graphics::DrawMode::stroke(3.0),
                &polygon.verts,
                OBSTACLE,
            )?;

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
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

        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &game_map.end_area.verts,
            color,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Draw vertices if selected
        if is_selected {
            self.draw_polygon_vertices(ctx, &game_map.end_area.verts)?;
        }

        Ok(())
    }

    fn draw_polygon_vertices(&self, ctx: &mut Context, vertices: &[Point2<f32>]) -> GameResult {
        for vertex in vertices {
            let mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                *vertex,
                5.0,
                0.01,
                graphics::WHITE,
            )?;

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        Ok(())
    }

    fn draw_actors(&self, ctx: &mut Context, actors: &[Actor]) -> GameResult<()> {
        for actor in actors {
            self.draw_actor(ctx, actor)?;
        }

        Ok(())
    }

    fn draw_actor(&self, ctx: &mut Context, actor: &Actor) -> GameResult<()> {
        if !actor.is_player() {
            self.draw_discovery_bar(ctx, actor.discovered_player, &actor.pos, actor.radius)?;
        }

        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [actor.pos.x, actor.pos.y],
            actor.radius,
            0.5,
            graphics::WHITE,
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

        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(left, top, width, height),
            graphics::Color::from_rgb(100, 100, 255),
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }
}
