use ggez::{
    GameResult,
    glam::vec2,
    graphics::{self, Color, Mesh},
};

pub struct EyeProperties {
    eye_width: f32,
    eye_height: f32,
    eye_spacing: f32,
    // randomized_offset: f32,
}

pub fn create_eye_properties(ctx: &ggez::Context) -> GameResult<(Mesh, Mesh)> {
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
    );

    let left_eye = graphics::Mesh::new_ellipse(
        ctx,
        graphics::DrawMode::fill(),
        vec2(second_x, eye_y),
        eye_properties.eye_width,
        eye_properties.eye_height,
        tolerance,
        Color::YELLOW,
    );
    Ok((right_eye.unwrap(), left_eye.unwrap()))
}
