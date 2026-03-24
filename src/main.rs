//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    Context, GameResult, event,
    glam::*,
    graphics::{self, Color},
};

struct MainState {
    ellipse: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let ellipse = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.,
            150.,
            2.0,
            Color::YELLOW,
        )?;
        Ok(MainState { ellipse })
        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     // This is basically where the center of the object should be. (0.0, 0.0) means center
        //     // Positive numbers go up and left, negative numbers down and right
        //     vec2(0., 0.),
        //     100.0,
        //     2.0,
        //     Color::WHITE,
        // )?;
        //
        // Ok(MainState { circle })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let dimensions = canvas.screen_coordinates().unwrap();
        canvas.draw(
            &self.ellipse,
            Vec2::new(dimensions.w / 2., dimensions.h / 2.),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
