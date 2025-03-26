use reqwest::blocking::get;
use reqwest::StatusCode;
use std::fs::File;
use std::io::copy;
use chrono::{Local, prelude::*};
use image::{GenericImageView, Pixel as ImagePixel, Rgba};
use crate::consts::*;
use crate::storm::Storm;

pub fn generate_url() -> String {
    // Get the current UTC time
    let local: DateTime<Utc> = Utc::now();
    println!("utc time: {}", local);

    // Extract components
    let mut year:i32 = local.year()as i32;
    let mut month:i32 = local.month() as i32;
    let mut day:i32 = local.day() as i32;
    let mut hour:i32 = local.hour() as i32;
    let mut minute: i32 = (local.minute() as i32/ 6) * 6 - 12; // Ensure minute is non-negative
    // Adjust hour, day, month, and year if minute is negative
    if minute < 0 {
        minute += 60;
        hour -= 1;
        if hour < 0 {
            hour += 24;
            day -= 1;
            if day < 1 {
                // Go to previous month
                month -= 1;
                if month < 1 {
                    month = 12;
                    year -= 1;
                }
                // Determine the last day of the previous month
                day = Utc.ymd(year, month as u32, 1).num_days_from_ce() as i32;
            }
        }
    }

    // Format the components into a string
    let formatted_time_former = format!("{:04}{:02}{:02}", year, month, day);
    let formatted_time_latter = format!("{:04}{:02}{:02}{:02}{:02}", year, month, day, hour, minute);
    let url_head = URL_HEAD;
    let url_middle = URL_MIDDLE;
    let url_end = URL_END;
    let entire_url = format!("{}{}{}{}{}", url_head, formatted_time_former, url_middle, formatted_time_latter, url_end);
    println!("latest radar image ulr: {}", entire_url);
    entire_url
}


pub fn print_storms(storm_list: &[Storm]) {
    let local = Local::now();
    println!("Observe Station: GuangZhou");
    println!("Process Time: {}", local);
    println!("Storm number in active: {}", storm_list.len());
    // Print header
    println!("{:<8} {:<15} {:<10} {:<20} {:<10}", "ID", "Distance (km)", "Compass", "Max Intensity (dBZ)", "Type");

    // Print storm data
    for storm in storm_list {
        println!(
            "{:<8} {:<15.2} {:<10} {:<20} {:<10}",
            storm.storm_id,
            storm.distance,
            azimuth_to_direction(storm.direction),
            storm.max_intensity,
            storm.storm_type
        );
    }
}


pub fn download_radar_image(image_url:&String, local_image_path:&str)
{
    match download_image(&image_url, local_image_path) {
        Ok(_) => println!("Image downloaded successfully."),
        Err(e) => {
            eprintln!("Error downloading image: {}", e);
            return;
        }
    }
}


fn download_image(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;

    if response.status() != StatusCode::OK {
        return Err(format!("Failed to download image: {}", response.status()).into());
    }

    let mut file = File::create(file_path)?;
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut file)?;

    Ok(())
}


fn azimuth_to_direction(azimuth: f64) -> &'static str {
    match azimuth {
        a if (0.0..=22.5).contains(&a) || (337.5..=360.0).contains(&a) => "N",
        a if (22.5..=67.5).contains(&a) => "NE",
        a if (67.5..=112.5).contains(&a) => "E",
        a if (112.5..=157.5).contains(&a) => "SE",
        a if (157.5..=202.5).contains(&a) => "S",
        a if (202.5..=247.5).contains(&a) => "SW",
        a if (247.5..=292.5).contains(&a) => "W",
        a if (292.5..=337.5).contains(&a) => "NW",
        _ => "Unknown",
    }
}


pub fn copy_legend(radar_img_path: &str, base_img_path: &str) {
    let radar_img = image::open(radar_img_path).expect("Failed to open radar image");
    let mut base_img = image::open(base_img_path).expect("Failed to open base image").to_rgba8();;

    let (radar_width, radar_height) = radar_img.dimensions();
    let (base_width, base_height) = base_img.dimensions();

    if radar_width != base_width || radar_height != base_height {
        panic!("Image dimensions do not match");
    }

    for x in radar_height..radar_width {
        for y in 0..radar_height {
            let pixel_color = radar_img.get_pixel(x, y).to_rgba();
            base_img.put_pixel(x, y, pixel_color);
        }
    }

    base_img.save(base_img_path).expect("Failed to save legend image");
}