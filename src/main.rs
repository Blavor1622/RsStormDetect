mod utils;
mod pixel;
mod storm;
mod consts;
use crate::consts::*;
use utils::{print_storms, copy_legend};
use pixel::filter_pixels_with_color;
use storm::{merge_pixels, generate_result_image};

fn main() {
    // Image paths
    let original_radar_image_path = "Z_RADR_I_Z9200_202404241348_P_DOR_SA_R_10_230_15.200.png";
    let base_image = String::from("base.png");
    let output_path = String::from("result.png");

    // Copy legend from input radar image to base image
    copy_legend(&original_radar_image_path, &base_image);

    // Constants variables
    let color_list = &COLOR_LIST;
    let radar_area = RADAR_AREA;
    let radar_center = RADAR_CENTER;

    // Get filtered echo pixels
    let filtered_pixels_list = filter_pixels_with_color(original_radar_image_path, &color_list, radar_area);

    // Storm analysis
    let mut storm_list = merge_pixels(&filtered_pixels_list, &radar_center);
    
    // Result image generation
    generate_result_image(&mut storm_list, &radar_center, &base_image, &output_path);

    // Print storm information list
    print_storms(&storm_list);
}

