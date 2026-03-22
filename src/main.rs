//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use crate::graphics::Color;
use ggez::{
    Context, GameResult, event,
    glam::*,
    graphics::{self, DrawParam},
};
use screen_size::get_primary_screen_size;

struct ScreenSize {
    width: f32,
    height: f32,
}

struct EyeProperties {
    eye_width: f32,
    eye_height: f32,
    eye_spacing: f32,
    eye_top_y: f32,
    eye_bottom_y: f32,
}

enum EyeState {
    Open,
    Closed,
}

struct Blinking {
    open_time: i32,
    close_time: i32,
    last_blink_time: i32,
    current_state: EyeState,
}

struct Scared {
    scared: bool,
    scared_start_time: i32,
    scared_duration: i32,
}

struct MainState {
    left_eye: graphics::Mesh,
    eye_properties: EyeProperties,
    screen_size: ScreenSize,
    solver_image: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (width, height) = get_primary_screen_size().expect("Screen size");
        let screen_size = ScreenSize {
            width: width as f32,
            height: height as f32,
        };

        // Declared these outside the struct so i don't need to repeat the calculation
        let eye_height = (screen_size.height / 5.) * 4.;
        let eye_y = (screen_size.height / 5.) * 4.;

        let eye_properties = EyeProperties {
            eye_width: screen_size.width / 3.75 + 20.,
            eye_height,
            eye_spacing: (screen_size.width / 6.) * 2.,
            eye_top_y: eye_y,
            eye_bottom_y: eye_y + eye_height,
        };

        let left_eye = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            vec2(100., 1000.),
            eye_properties.eye_height,
            eye_properties.eye_width,
            1.0,
            Color {
                r: 254.,
                g: 221.,
                b: 0.,
                a: 1.,
            },
        )?;

        let solver_image_url = "https://i.quotev.com/g76gnudmst2q.jpg";
        let img_bytes = match reqwest::blocking::get(solver_image_url) {
            Ok(x) => match x.bytes() {
                Ok(bytes) => bytes,
                Err(_) => panic!("Could not convert to bytes"),
            },
            Err(_) => panic!("Error"),
        };

        let solver_image = crate::graphics::Image::from_bytes(ctx, &img_bytes)?;

        Ok(MainState {
            left_eye,
            eye_properties,
            screen_size,
            solver_image,
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

        canvas.draw(&self.left_eye, DrawParam::new());
        // canvas.draw(&self.solver_image, Vec2::new(380., 380.));

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
