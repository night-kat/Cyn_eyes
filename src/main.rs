//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

#[derive(Debug)]
struct EyeProperties {
    eye_width: f32,
    eye_height: f32,
    eye_spacing: f32,
    // randomized_offset: f32,
}

use ggez::event;
use ggez::glam::{Vec2, vec2};
use ggez::graphics::{DrawMode, Mesh};
use ggez::{
    Context, GameResult,
    graphics::{self, Color},
};
use rand::random_range;

struct MainState {
    right_eye: Mesh,
    left_eye: Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (w, h) = graphics::GraphicsContext::drawable_size(&ctx.gfx);

        // let randomized_offset = random_range(1..80) as f32;
        let eye_properties = EyeProperties {
            eye_width: w * 0.15,
            eye_height: (h * 0.45),
            eye_spacing: h / 2.,
            // randomized_offset,
        };

        let initial_x = w * 0.25;
        let second_x = initial_x + eye_properties.eye_width + eye_properties.eye_spacing;
        let eye_y = h / 2.;

        // No reason for this value in particular, it is just what I found
        // works well
        let tolerance = 2.0;

        let right_eye = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            // Place one eye at one quarter of the width (left side) and half the height
            vec2(initial_x, eye_y),
            eye_properties.eye_width,
            eye_properties.eye_height,
            tolerance,
            Color::YELLOW,
        )?;

        let left_eye = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            vec2(second_x, eye_y),
            eye_properties.eye_width,
            eye_properties.eye_height,
            tolerance,
            Color::YELLOW,
        )?;

        Ok(MainState {
            right_eye,
            left_eye,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.left_eye, Vec2::new(0., 0.));
        canvas.draw(&self.right_eye, Vec2::new(0., 0.));

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
