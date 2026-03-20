use std::panic;

use screen_size::get_primary_screen_size;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solver_image_url = "https://i.quotev.com/g76gnudmst2q.jpg";

    struct Screen {
        width: u64,
        height: u64,
    }

    let screen_size = match get_primary_screen_size() {
        Ok((width, height)) => Screen { width, height },
        Err(_) => panic!(),
    };

    let solver_img = image::load_from_memory(match reqwest::blocking::get(solver_image_url) {
        Ok(result) => match result.bytes() {
            Ok(img_bytes) => img_bytes,
            Err(_) => panic!(),
        },
        Err(_) => panic!(),
    });

    Ok(())
}


// use std::{error::Error, fs};
//
// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     match fs::exists("solver_image") {
//         Ok(_) => Ok(()),
//         Err() => Error,
//     };
//     let solver_image_url = "https://i.quotev.com/g76gnudmst2q.jpg";
//     let img_bytes = reqwest::blocking::get(solver_image_url)?.bytes()?;
//     let solver_image = image::load_from_memory(&img_bytes)?;
//     let _ = solver_image.save("solver_image.jpg");
//     Ok(())
// }
//
