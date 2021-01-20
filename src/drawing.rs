use crate::{actor::Actor, colors, fov::FieldOfView, game_map::GameMap, state::State};
use ggez::{graphics, Context, GameResult};

pub fn draw_all(ctx: &mut Context, state: &State) -> GameResult<()> {
    // TODO: These should re-use the meshes instead of remaking each time
    draw_all_fov(ctx, &state.actors)?;
    draw_obstacles(ctx, &state.game_map)?;
    draw_end_area(ctx, &state.game_map)?;

    // Draw all actors
    for actor in &state.actors {
        actor.draw(ctx)?;
    }

    Ok(())
}

fn draw_all_fov(ctx: &mut Context, actors: &[Actor]) -> GameResult<()> {
    for actor in actors {
        let color = if actor.is_player() {
            colors::PLAYER_VISIBLE_AREA
        } else {
            colors::GUARD_VISIBLE_AREA
        };
        draw_fov(ctx, &*actor.fov, color)?;
    }

    Ok(())
}

fn draw_fov(ctx: &mut Context, fov: &dyn FieldOfView, color: graphics::Color) -> GameResult<()> {
    let visible_area = match fov.get_visible_area() {
        Some(polygon) => polygon,
        None => return Ok(()),
    };

    if visible_area.verts.len() < 3 {
        return Ok(());
    }

    let mesh =
        graphics::Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &visible_area.verts, color)?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}

fn draw_obstacles(ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
    for polygon in &game_map.obstacles {
        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(3.0),
            &polygon.verts,
            colors::OBSTACLE,
        )?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_end_area(ctx: &mut Context, game_map: &GameMap) -> GameResult<()> {
    let mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &game_map.end_area.verts,
        colors::END_AREA,
    )?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())
}
