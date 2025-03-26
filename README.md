# RustRADSI(Weather Radar Storm Identification with Rust)

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Description

This is a rust project for identifying active storms from input weather radar image in reflectivity mode. The project will generate a active storm information list, including direction, distance from radar center, maximum reflectivity and storm category.

## Features  
- **User-friendly** – Simple and easy for setting up.  
- **Customizable parameters** – Allows fine-tuning of internal parameters for better identification results.
- **Well-structured code** – Clean and organized codes for easy maintenance and extensibility.

## Result Preview
### Input radar image
![Input radar image](data\input\Z_RADR_I_Z9200_202404241348_P_DOR_SA_R_10_230_15.200.png)
### Analysis result image
![Analysis result image](data\output\result.png)
### Storm information list
![Storm information list](data\storm_list_screenshot.png)
## Installation

### Prerequisites
- Rust 1.70+ ([Installation Guide](https://www.rust-lang.org/tools/install))
- Cargo (Rust's package manager)

```sh
# Clone the repository
git clone https://github.com/Blavor1622/RustRADSI.git

# Navigate into the directory
cd RustRADSI

# Install dependencies
cargo build

# Run the program
cargo run
