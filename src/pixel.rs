use image::{GenericImageView, Pixel as ImagePixel, Rgba};
use crate::consts::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
    pub color: Rgba<u8>,
    pub intensity: u32,
}

fn range_match(color: &Rgba<u8>, target_color: &Rgba<u8>, range_width: u8) -> bool {
    target_color
        .channels()
        .iter()
        .zip(color.channels().iter())
        .all(|(&t, &c)| t.saturating_sub(range_width) <= c && c <= t.saturating_add(range_width))
}

fn acquire_intensity(pixel_color: &Rgba<u8>) -> u32 {
    match pixel_color {
        //c if range_match(c, &Rgba([0, 160, 224, 255]), 10) => 10,
        c if range_match(c, &Rgba([0, 0, 246, 255]), 10) => 15,
        c if range_match(c, &Rgba([0, 254, 0, 255]), 10) => 20,
        c if range_match(c, &Rgba([0, 200, 0, 255]), 10) => 25,
        c if range_match(c, &Rgba([0, 144, 0, 255]), 10) => 30,
        c if range_match(c, &Rgba([254, 254, 0, 255]), 10) => 35,
        c if range_match(c, &Rgba([230, 192, 0, 255]), 10) => 40,
        c if range_match(c, &Rgba([254, 144, 0, 255]), 10) => 45,
        c if range_match(c, &Rgba([254, 0, 0, 255]), 10) => 50,
        c if range_match(c, &Rgba([166, 0, 0, 255]), 10) => 55,
        c if range_match(c, &Rgba([100, 0, 0, 255]), 10) => 60,
        c if range_match(c, &Rgba([254, 0, 254, 255]), 10) => 65,
        c if range_match(c, &Rgba([152, 84, 200, 255]), 10) => 70,
        _ => 0,
    }
}

pub fn filter_pixels_with_color(
    image_path: &str,
    color_list: &[Rgba<u8>],
    radar_area: (u32, u32),
) -> Vec<Pixel> {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    let (radar_width, radar_height) = radar_area;
    
    if radar_width > width || radar_height > height {
        panic!("Radar area exceeds image dimensions");
    }

    let mut filtered_result = Vec::new();

    for x in 0..radar_width {
        for y in 0..radar_height {
            let pixel_color = img.get_pixel(x, y).to_rgba();
            for target_color in color_list {
                if range_match(&pixel_color, target_color, DELTA) {
                    let intensity = acquire_intensity(&pixel_color);
                    filtered_result.push(Pixel {
                        x: x as i32,
                        y: y as i32,
                        color: pixel_color,
                        intensity,
                    });
                }
            }
        }
    }

    filtered_result
}