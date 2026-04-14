mod eyes;

use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics::Mesh;
use ggez::{
    Context, GameResult,
    graphics::{self},
};

use crate::eyes::create_eye_properties;

#[derive(Debug)]

struct MainState {
    right_eye: Mesh,
    left_eye: Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (right_eye, left_eye) = create_eye_properties(ctx).unwrap();
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
