use image::{DynamicImage, GenericImageView, Pixel};
use rand::Rng;
use std::cmp::min;

// return image dimensions as u32 and f32
fn get_image_dimensions(img: &DynamicImage) -> (u32, u32, f32, f32) {
    let h = img.height();
    let w = img.width();
    (h, w, h as f32, w as f32)
}

pub fn generate_realistic_locations(mut count: u32) -> Vec<String> {
    let base_img: DynamicImage = image::open("tests/redis/earth_night.jpg").unwrap();
    let (h, w, hf, wf) = get_image_dimensions(&base_img);

    let mut locations_vec = vec![];
    let mut rng = rand::thread_rng();
    while count != 0 {
        let x = rng.gen_range(0..w - 1);
        let y = rng.gen_range(0..h - 1);
        let luma = base_img.get_pixel(x, y).to_luma()[0];
        if luma > 200 {
            let lon = (x as f32 / wf) * 360.0 - 180.0;
            let lat = -((y as f32 / hf) * 180.0 - 90.0);
            locations_vec.push(format!("{lat}|{lon}"));
            count -= 1;
        }
    }

    locations_vec
}

#[allow(dead_code)]
pub fn generate_locations(mut count: u32) -> Vec<String> {
    let img: DynamicImage = image::open("static/img/map.jpeg").unwrap();
    let (h, w, hf, wf) = get_image_dimensions(&img);
    let ocean_color = img.get_pixel(200, 400);

    let mut locations_vec = vec![];
    let mut rng = rand::thread_rng();
    while count != 0 {
        let lon: f32 = rng.gen_range(-180.0..180.0);
        let lat: f32 = rng.gen_range(-90.0..90.0);

        // rounding to six decimal places
        let lon = (lon * 1_000_000.0).round() / 1_000_000.0;
        let lat = (lat * 1_000_000.0).round() / 1_000_000.0;

        let x = wf / 2.0 + (wf / 360.0) * lon;
        let y = hf - (hf / 2.0 + (hf / 180.0) * lat);
        if img.get_pixel(min(w - 1, x as u32), min(h - 1, y as u32)) != ocean_color {
            locations_vec.push(format!("{lat}|{lon}"));
            count -= 1;
        }
    }

    locations_vec
}
