//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

#[derive(Debug)]
struct EyeProperties {
    eye_width: f32,
    eye_height: f32,
}
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
        let (w, h) = graphics::GraphicsContext::drawable_size(&ctx.gfx);
        let eye_properties = EyeProperties {
            eye_width: w * 0.20,
            eye_height: (h * 0.5),
        };

        dbg!(&eye_properties);

        let ellipse = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            // Place one eye at one quarter of the width (left side) and half the height
            vec2(w * (1. / 4.), h / 2.),
            eye_properties.eye_width,
            eye_properties.eye_height,
            2.0,
            Color::YELLOW,
        )?;
        Ok(MainState { ellipse })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.ellipse, Vec2::new(0., 0.));

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
