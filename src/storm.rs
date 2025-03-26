use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use nalgebra::Point2;
use rusttype::{Font, Scale};
use std::collections::HashSet;
use std::f32::consts::PI;
use crate::consts::*;
use crate::pixel::Pixel;

#[derive(Debug)]
pub struct Storm {
    pub storm_id: u32,
    pub intensity_center: Point2<i32>,
    pub distance: f64,
    pub direction: f64,
    pub storm_type: String,
    pub max_intensity: u32,
    pub pixels: Vec<Pixel>,
}

pub fn merge_pixels(pixel_list: &[Pixel], radar_center: &Point2<f64>) -> Vec<Storm> {
    let mut storm_list = Vec::new();
    let mut visited = HashSet::new();

    for &pixel in pixel_list {
        if !visited.contains(&pixel) {
            let mut merged_pixel = vec![pixel];
            visited.insert(pixel);
            let mut stack = vec![pixel];

            while let Some(current_pixel) = stack.pop() {
                let neighbors = get_adjacent_pixels(current_pixel, pixel_list);
                for &neighbor in neighbors.iter() {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        merged_pixel.push(neighbor);
                        stack.push(neighbor);
                    }
                }
            }

            let max_ref = acquire_maximum_reflectivity(&merged_pixel);
            if merged_pixel.len() > MIN_SIZE && max_ref >= MIN_INTENSITY {
                let inten_center: nalgebra::OPoint<i32, nalgebra::Const<2>> = calculate_intensity_center(&merged_pixel);
                let distance = calculate_herb_center_distance(&inten_center, &radar_center);
                let inten_center_64 = Point2::new(inten_center.x as f64, inten_center.y as f64);
                let angle_azimuth = calculate_azimuth_degrees(&inten_center_64, &radar_center);
                let storm = Storm {
                    storm_id: 0, // Assign a temporary ID, it will be updated later
                    intensity_center: inten_center,
                    distance: distance,
                    direction: angle_azimuth,
                    storm_type: String::from("default"), // zero stands for default type
                    max_intensity: max_ref,
                    pixels: merged_pixel,
                };
                storm_list.push(storm);
            }
        }
    }

    // Sort storm_list by distance
    storm_list.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // Assign storm_id based on sorted order
    for (id, storm) in storm_list.iter_mut().enumerate() {
        storm.storm_id = (id + 1) as u32;
    }

    storm_list
}


pub fn generate_result_image(storms: &mut [Storm], radar_center: &Point2<f64>, input_image_path: &str, output_image_path: &str) {
    let mut img = image::open(input_image_path).expect("Failed to open image").to_rgba8();
    let radar_center_point = Point2::new(radar_center.x.round() as i32, radar_center.y.round() as i32);

    for storm in storms {
        // Draw each pixel in the storm's pixels vector in a specific color (e.g., white)
        for pixel in &storm.pixels {
            img.put_pixel(pixel.x as u32, pixel.y as u32, pixel.color);
        }

        // Draw the intensity center in green
        img.put_pixel(storm.intensity_center.x as u32, storm.intensity_center.y as u32, INTENSITY_CENTER_COLOR);
        
        // Calculate ellipse parameters
        let (major_axis_length, farthest_pixel) = longest_distance_from_center(storm);
        let storm_center_f64 = Point2::new(storm.intensity_center.x as f64, storm.intensity_center.y as f64);
        let farthest_pixel_f64 = Point2::new(farthest_pixel.x as f64, farthest_pixel.y as f64);
        let major_axis_angle = calculate_angle_from_origin(&storm_center_f64, &farthest_pixel_f64);

        // Define the major axis line
        let major_axis_line = (storm.intensity_center, farthest_pixel);

        // Calculate the length of the minor axis
        let minor_axis_length = farthest_distance_from_line(storm, major_axis_line);

        let mut major_pixel_num = 0;
        for pixel in &storm.pixels {
            if pixel.intensity >= 45 {
                major_pixel_num += 1;
            }
        }
        if major_pixel_num >= MAJOR_PIXEL_THRESHOLD {
            // Draw the rotated ellipse
            draw_rotated_ellipse_mut(
                &mut img,
                (storm.intensity_center.x as i32, storm.intensity_center.y as i32),
                major_axis_length.round() as i32,
                minor_axis_length.round() as i32,
                major_axis_angle,
                Rgba([118, 95, 255, 255]),
            );
        }
        let eccentricity = calculate_eccentricity(major_axis_length, minor_axis_length);
        if eccentricity >= TYPE_THRESHOLD && eccentricity < 1.0
        {
            storm.storm_type = String::from("multicell");
        }else
        {
            storm.storm_type = String::from("single cell");
        }
        // Draw the storm ID
        let font_data: &[u8] = include_bytes!("../assets/DejaVuSans.ttf"); // Use the DejaVuSans font
        let font = Font::try_from_bytes(font_data).expect("Failed to load font");
        let scale = Scale { x: 18.0, y: 18.0 };
        let text = format!("#{}", storm.storm_id);
        let text_x = storm.intensity_center.x + 15;
        let text_y = storm.intensity_center.y - 15;
        draw_text_mut(&mut img, Rgba([255, 255, 255, 255]), text_x, text_y, scale, &font, &text);
        // Optionally, draw lines to the radar center
        draw_line(&mut img, radar_center_point, storm.intensity_center, CONNECTION_LINE_COLOR);
    }

    img.save(output_image_path).expect("Failed to save image");
}


fn acquire_maximum_reflectivity(herb: &[Pixel]) -> u32 {
    herb.iter().map(|pixel| pixel.intensity as u32).max().unwrap_or(0)
}

fn get_adjacent_pixels(pixel: Pixel, pixel_list: &[Pixel]) -> Vec<Pixel> {
    pixel_list
        .iter()
        .cloned()
        .filter(|&neighbor| {
            (pixel.x - neighbor.x).abs() <= ADJACENT_THRESHOLD && (pixel.y - neighbor.y).abs() <= ADJACENT_THRESHOLD
        })
        .collect()
}

fn calculate_intensity_center(herb: &[Pixel]) -> Point2<i32> {
    let mut x_center = 0.0;
    let mut y_center = 0.0;
    let mut sum_weight = 0.0;

    for pixel in herb {
        if pixel.intensity >= MIN_INTENSITY {
            let weight = pixel.intensity as f64;
            sum_weight += weight;
            x_center += pixel.x as f64 * weight;
            y_center += pixel.y as f64 * weight;
        }
    }

    if sum_weight != 0.0 {
        x_center /= sum_weight;
        y_center /= sum_weight;
    }

    Point2::new(x_center.round() as i32, y_center.round() as i32)
}

fn calculate_herb_center_distance(storm_center: &Point2<i32>, radar_center: &Point2<f64>) -> f64 {
    let storm_center_f64 = Point2::new(storm_center.x as f64, storm_center.y as f64);
    nalgebra::distance(&storm_center_f64, radar_center) * DISTANCE_RATIO
}


fn calculate_angle_from_origin(storm_center: &Point2<f64>, radar_center: &Point2<f64>) -> f64 {
    let x = storm_center.x - radar_center.x;
    let y = storm_center.y - radar_center.y;
    y.atan2(x) // atan2(y, x) returns the angle in radians
}

fn calculate_azimuth_degrees(storm_center: &Point2<f64>, radar_center: &Point2<f64>) -> f64 {
    let x = storm_center.x - radar_center.x;
    let y =  radar_center.y - storm_center.y;
    let len = (x * x + y * y).sqrt();
    let mut azimuth = 0.0;
    if x > 0.0 && y > 0.0
    {
        let sin_value = y / len;
        let afa = sin_value.asin() * 180.0 / std::f64::consts::PI;
        azimuth = 90.0 - afa;
    }
    else if x < 0.0 && y > 0.0
    {
        let sin_value = y / len;
        let afa = sin_value.asin() * 180.0 / std::f64::consts::PI;
        azimuth = 270.0 + afa;
    }else if x < 0.0 && y < 0.0
    {
        let sin_value = -y / len;
        let afa = sin_value.asin() * 180.0 / std::f64::consts::PI;
        azimuth = 270.0 - afa;
    }else if x > 0.0 && y < 0.0
    {
        let sin_value = -y / len;
        let afa = sin_value.asin() * 180.0 / std::f64::consts::PI;
        azimuth = 90.0 + afa;
    }else if x == 0.0 && y > 0.0
    {
        azimuth = 0.0
    }else if x == 0.0 && y < 0.0
    {
        azimuth = 180.0
    }else if x > 0.0 && y == 0.0
    {
        azimuth = 90.0
    }else if x < 0.0 && y == 0.0
    {
        azimuth = 270.0
    }
    azimuth
}


fn longest_distance_from_center(storm: &Storm) -> (f64, Point2<i32>) {
    let (max_distance, farthest_pixel) = storm.pixels.iter()
        .filter_map(|pixel| {
            if pixel.intensity >= 45 {
                let dx = (pixel.x - storm.intensity_center.x) as f64;
                let dy = (pixel.y - storm.intensity_center.y) as f64;
                Some(((dx * dx + dy * dy).sqrt(), Point2::new(pixel.x, pixel.y)))
            } else {
                None
            }
        })
        .fold((0.0, Point2::new(0, 0)), |(max_dist, max_pixel), (dist, pixel)| {
            if dist > max_dist {
                (dist, pixel)
            } else {
                (max_dist, max_pixel)
            }
        });
    (max_distance, farthest_pixel)
}

fn farthest_distance_from_line(storm: &Storm, line: (Point2<i32>, Point2<i32>)) -> f64 {
    let (start, end) = line;
    let (x0, y0) = (start.x as f64, start.y as f64);
    let (x1, y1) = (end.x as f64, end.y as f64);
    
    storm.pixels.iter()
        .filter_map(|pixel| {
            if pixel.intensity >= 45 {
                let (x, y) = (pixel.x as f64, pixel.y as f64);
                let distance = ((y1 - y0) * x - (x1 - x0) * y + x1 * y0 - y1 * x0).abs() / ((y1 - y0).powi(2) + (x1 - x0).powi(2)).sqrt();
                Some(distance)
            } else {
                None
            }
        })
        .fold(0.0, f64::max)
}

fn draw_rotated_ellipse_mut(
    img: &mut RgbaImage,
    center: (i32, i32),
    major_axis: i32,
    minor_axis: i32,
    angle_rad: f64,
    color: Rgba<u8>,
) {
    let mut ellipse_points = vec![];
    for i in 0..360 {
        let theta = i as f64 * PI as f64 / 180.0;
        let x = (major_axis as f64 * theta.cos() * angle_rad.cos() - minor_axis as f64 * theta.sin() * angle_rad.sin()) + center.0 as f64;
        let y = (major_axis as f64 * theta.cos() * angle_rad.sin() + minor_axis as f64 * theta.sin() * angle_rad.cos()) + center.1 as f64;
        ellipse_points.push((x as i32, y as i32));
    }

    for (x, y) in ellipse_points {
        if x >= 0 && y >= 0 {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

fn calculate_eccentricity(major_axis: f64, minor_axis: f64) -> f64 {
    (1.0 - (minor_axis.powi(2) / major_axis.powi(2))).sqrt()
}

fn draw_line(img: &mut RgbaImage, start: Point2<i32>, end: Point2<i32>, color: Rgba<u8>) {
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = start.x;
    let mut y = start.y;

    loop {
        img.put_pixel(x as u32, y as u32, color);

        if x == end.x && y == end.y {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
