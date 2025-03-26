use image::Rgba;
use nalgebra::Point2;

// Use lazy_static or once_cell for heap-allocated data types
use once_cell::sync::Lazy;

pub const DELTA: u8 = 10;
pub const RADAR_CENTER: Point2<f64> = Point2::new(300.0, 300.0);
pub const RADAR_AREA: (u32, u32) = (599, 599);
pub const TYPE_THRESHOLD: f64 = 0.88;

pub const URL_HEAD: &str = "http://tqyb.com.cn/data/radar/gz/19/";
pub const URL_MIDDLE: &str = "/Z9200_";
pub const URL_END: &str = "Z_PPI_02_19.png";

pub const MIN_SIZE: usize = 40;
pub const MIN_INTENSITY: u32 = 45;

pub const ADJACENT_THRESHOLD: i32 = 2;
pub const MAJOR_PIXEL_THRESHOLD: i32 = 50;

pub const INTENSITY_CENTER_COLOR: Rgba<u8> = Rgba([0, 255, 0, 255]);
pub const CONNECTION_LINE_COLOR: Rgba<u8> = Rgba([105, 131, 255, 255]);
// Lazy static initialization for vectors
pub static COLOR_LIST: Lazy<Vec<Rgba<u8>>> = Lazy::new(|| vec![
    Rgba([0, 0, 246, 255]),
    Rgba([0, 254, 0, 255]),
    Rgba([0, 200, 0, 255]),
    Rgba([0, 144, 0, 255]),
    Rgba([254, 254, 0, 255]),
    Rgba([230, 192, 0, 255]),
    Rgba([254, 144, 0, 255]),
    Rgba([254, 0, 0, 255]),
    Rgba([166, 0, 0, 255]),
    Rgba([100, 0, 0, 255]),
    Rgba([254, 0, 254, 255]),
    Rgba([152, 84, 200, 255]),
]);

pub const DISTANCE_RATIO: f64 = 200.0 / (300.0 - 65.0) as f64;