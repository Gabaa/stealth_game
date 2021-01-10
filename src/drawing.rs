use crate::{colors, graphics, Actor, Context, FieldOfView, GameMap, GameResult, State};

pub fn draw_all(ctx: &mut Context, state: &State) -> GameResult<()> {
    // TODO: These should re-use the meshes instead of remaking each time
    draw_all_fov(ctx, &state.player, &state.guards)?;
    draw_obstacles(ctx, &state.game_map)?;
    draw_end_area(ctx, &state.game_map)?;
    draw_actors(ctx, &state.player, &state.guards)
}

fn draw_all_fov(ctx: &mut Context, player: &Actor, guards: &[Actor]) -> GameResult<()> {
    for guard in guards {
        draw_fov(ctx, &guard.fov, colors::GUARD_VISIBLE_AREA)?;
    }

    draw_fov(ctx, &player.fov, colors::PLAYER_VISIBLE_AREA)
}

fn draw_fov(
    ctx: &mut Context,
    fov: &Box<dyn FieldOfView>,
    color: graphics::Color,
) -> GameResult<()> {
    if fov.get_visible_area().verts.len() < 3 {
        return Ok(());
    }

    let mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &fov.get_visible_area().verts,
        color,
    )?;

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

fn draw_actors(ctx: &mut Context, player: &Actor, guards: &[Actor]) -> GameResult<()> {
    for guard in guards {
        draw_actor(ctx, guard)?;
    }

    draw_actor(ctx, player)
}

fn draw_actor(ctx: &mut Context, actor: &Actor) -> GameResult<()> {
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
