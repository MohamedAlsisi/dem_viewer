// PDCP and MA Rust exam 2025
// instructions: https://docs.google.com/document/d/1715_OBLBiObkOKHCpMefYCrTg0mXt-c7hjM171OzVSU/edit?tab=t.0
// Last update 2/abril/2025

//Import libraries 
// Initial libraries and the ones to import the ASC file
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//Libraries to visualize as a grayscale DEM
use image::{GrayImage, Luma};

//Libraries to visualize with colorgrad
use image::{RgbImage, Rgb};
use colorgrad::preset::turbo;
use colorgrad::Gradient; 

// Library for hillshade 
use std::f64::consts::PI;

use image::{DynamicImage, RgbaImage, imageops};

fn main() {
    // Bonjour
    println!("\n\tHola!!\n");
    println!("This code will compute several images to visualize the elevation map");
    println!("First 4 iamges are computes that are the following:");
    println!("  - grayscale image");
    println!("  - colored image");
    println!("  - grayscale + shadehill image");
    println!("  - colored + shadehill image");

    // READING THE ASC FILE 
    // Input the specific path where the file is found. (see bellow the auxiliary function created to read the file)
    // Return the metdata (the first lines of the ASC) and the "grid" which are the real values in the grid
    let path = "/Users/mohamedalsisi/Downloads/MIR/Rust/0925_6225/LITTO3D_FRA_0928_6225_20150529_LAMB93_RGF93_IGN69/MNT1m/LITTO3D_FRA_0928_6225_MNT_20150128_LAMB93_RGF93_IGN69.asc"; // Update with your ASC file path
    let (metadata, grid) = read_asc_file(path).expect("Failed to read ASC file, please make sure everything is okay");

    // reading the size from the metdata info
    let ncols = metadata["ncols"] as u32;
    let nrows = metadata["nrows"] as u32;
    // Find min and max elevation for normalization that will come later
    let min_val = grid.iter().flatten().cloned().fold(f64::INFINITY, f64::min);
    let max_val = grid.iter().flatten().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Define color gradient (using turbo as suggested in the exam instructions :D )
    //let grad = turbo();
    let grad = turbo(); // Set the colormap to Inferno


    // Create grayscale and color images, it uses the size from the metdata taken before for the image size
    let mut grayscale_img = GrayImage::new(ncols, nrows);
    let mut color_img = RgbImage::new(ncols, nrows);
    let mut hillshade_grayscale_image = GrayImage::new(ncols, nrows);
    let mut hillshade_color_img = RgbImage::new(ncols, nrows);


    // HILLSHADE
    // Hillshade parameters
    let azimuth = 315.0;
    let altitude = 45.0;
    let cellsize = metadata["cellsize"];
    let z_factor = 1.0;

    // computing illumination angle for Hillshade
    let zenith_rad = (90.0 - altitude)*PI/180.0; //converted to radians in one go

    // computing illumination direction for Hillshade
    let mut azimuth_math = 360.0 - azimuth + 90.0; // right angle
    if azimuth_math > 360.0 {
        azimuth_math = azimuth_math - 360.0
    }
    azimuth_math = azimuth_math*PI/180.0; // converting to radians (using same variable no need to create new one)

    println!("\nHillshade calculation made with values:");
    println!("\tazimuth = {azimuth}");
    println!("\taltitude = {altitude} ");
    println!("\tcellsize = {cellsize}");
    println!("\tNo padding on the border elements used");
    println!("\n Computing images please wait\n");

    for (y, row) in grid.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            // Normalize value (0 to 1 range)
            let normalized = (val - min_val) / (max_val - min_val);

            // Grayscale (0 to 255 intensity)
            let gray_value = (normalized * 255.0) as u8;
            grayscale_img.put_pixel(x as u32, y as u32, Luma([gray_value]));

            // Colorized DEM using gradient
            let color = grad.at(normalized as f32); 
            color_img.put_pixel(x as u32,y as u32, Rgb([(color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8]));

            // HILLDSHADE
            let mut hillshade;
            // hillsahde will use a 3x3 neighbourhood soo this if statement makes the border elements equal to a constant value to avoid size issues
            if x == 0 || x == 1 || x == (nrows as usize) - 1 || x == (nrows as usize) - 2 || 
                y == 0 || y == 1 || y == (ncols as usize) - 1 || y == (ncols as usize) - 2 {
                    hillshade = 127.0; //border elements will look black (maybe try 255)
            } else {
                // computing slope and aspect for Hillshade
                    let dzdx = ((grid[y-1][x+1] + 2.0*grid[y][x+1] + grid[y+1][x+1]) - (grid[y-1][x-1] + 2.0*grid[y][x-1] + grid[y+1][x-1])) / (8.0*cellsize);
                    let dzdy = ((grid[y+1][x-1] + 2.0*grid[y+1][x]+ grid[y+1][x+1])-(grid[y-1][x-1] + 2.0*grid[y-1][x]+ grid[y-1][x+1]))/(8.0*cellsize);
                    let slope_rad = (z_factor*(dzdx*dzdx + dzdy*dzdy).sqrt()).atan();

                    let mut aspect_ratio = 0.0;
                    if dzdx != 0.0 {
                        aspect_ratio = dzdy.atan2(-dzdx);
                        if aspect_ratio < 0.0 {
                            aspect_ratio = 2.0 * PI + aspect_ratio;
                        }
                    }else {
                        if dzdy > 0.0 {
                            aspect_ratio = PI/2.0;
                        } else if dzdy < 0.0 {
                            aspect_ratio = PI*2.0 - PI/2.0
                        } else {
                            aspect_ratio = aspect_ratio;
                        }
                    }    
                    hillshade = 255.0 * ((zenith_rad.cos() * slope_rad.cos()) + (zenith_rad.sin() * slope_rad.sin() * (azimuth_math - aspect_ratio).cos()));
                    hillshade = hillshade.max(0.0).min(255.0); // Ensure valid range
            }
            //let shading = hillshade / 255.0;
            //let shading = (hillshade / 255.0).max(0.0).min(1.0); // Clamp to avoid negatives

            let shaded_r = (color.r as f64 * hillshade) as u8;
            let shaded_g = (color.g as f64 * hillshade) as u8;
            let shaded_b = (color.b as f64 * hillshade) as u8;
            
            //println!("Hillshade value at ({}, {}): {}", x, y, shading);

            hillshade_color_img.put_pixel(x as u32, y as u32, Rgb([shaded_r, shaded_g, shaded_b]));

            let hillshade_gray_value = (normalized * hillshade) as u8;
            hillshade_grayscale_image.put_pixel( x as u32, y as u32 ,Luma([hillshade_gray_value]));
        }
    }

    println!("Images computed and saved as follows:");
    // Save images
    grayscale_img.save("output_grayscale.png").expect("Failed to save grayscale image");
    color_img.save("output_color.png").expect("Failed to save color image");
    hillshade_color_img.save("output_hillshaded_color.png").expect("Failed to save hillshade color image");
    hillshade_grayscale_image.save("output_hillshade_grayscale_image.png").expect("Failed to save hillsahde grayscale image");

    println!("Grayscale DEM saved as output_grayscale.png");
    println!("Color DEM saved as output_color.png");
    println!("Hillshade grayscale image saved as hillshade_grayscale_image.png");
    println!("Hillshade colored image saved as hillshade_color_img.png");
    println!("\nPlease visualize the images by opening the files :)");

    // EXTRA FEATURE --------------------------------------------------------------------------
    println!("\n Now in this part the additional features are made. \nThe additional feature is a display of the elevations isolines ");
    // Call to generate contour lines, here you specify how many contour lines you'd like (e.g., 10 contours)
    let num_contours = 10; 
    println!("Amount of countour elvations lines: {num_contours}");
    let mut hillshade_with_contours = hillshade_color_img.clone();
    
    generate_contours(&grid, ncols, nrows, min_val, max_val, num_contours, &mut hillshade_with_contours);

    // Save the image with contour lines directly overlayed on the hillshade
    hillshade_with_contours.save("output_with_hillshade_and_contours.png").expect("Failed to save combined image with hillshade and contours");

    println!("Image with countour lines saved as output_with_hillshade_and_contours.png");

    let composite = combine_images_in_memory(
        &grayscale_img,
        &color_img,
        &hillshade_with_contours,
        &hillshade_color_img,
    );

    composite.save("composite.png").expect("Failed to save hillsahde grayscale image");

    let _ = open::that("output_composite.png");
    println!("\nFInal image with all the results saved as composite.png, please open this to visualize it! ;D");
    // BYE BYE BYE
    println!("\n\tBYEEE\n")
}

// Function to read ASC file and return metadata + elevation grid
fn read_asc_file<P>(filename: P) -> io::Result<(std::collections::HashMap<String, f64>, Vec<Vec<f64>>)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut metadata = std::collections::HashMap::new();
    let mut grid = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i < 6 {
            let parts: Vec<&str> = line.split_whitespace().collect();
            metadata.insert(parts[0].to_string(), parts[1].parse::<f64>().unwrap());
        } else {
            let row: Vec<f64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            grid.push(row);
        }
    }

    Ok((metadata, grid))
}

fn generate_contours(
    grid: &Vec<Vec<f64>>,
    ncols: u32,
    nrows: u32,
    min_val: f64,
    max_val: f64,
    num_contours: u32,
    image: &mut RgbImage
) {
    let contour_interval = (max_val - min_val) / (num_contours as f64);

    
    let bright_dark_red = Rgb([139, 0, 0]); // Dark red color

    // Iterate over the grid and draw contour lines directly on the hillshade image
    for contour_level in 0..num_contours {
        let contour_value = min_val + contour_level as f64 * contour_interval;

        for y in 1..(nrows - 1) {
            for x in 1..(ncols - 1) {
                let value = grid[y as usize][x as usize];

                // Check if current cell is near a contour line
                if (value >= contour_value && grid[(y - 1) as usize][x as usize] < contour_value) ||
                   (value <= contour_value && grid[(y + 1) as usize][x as usize] > contour_value) ||
                   (value >= contour_value && grid[y as usize][(x - 1) as usize] < contour_value) ||
                   (value <= contour_value && grid[y as usize][(x + 1) as usize] > contour_value) {
                    
                    // Draw a 3x3 block to make the line thicker
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as i32 + dx) as u32;
                            let ny = (y as i32 + dy) as u32;
                            if nx < ncols && ny < nrows {
                                image.put_pixel(nx, ny, bright_dark_red);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn combine_images_in_memory(
    grayscale_img: &GrayImage,
    color_img: &RgbImage,
    hillshade_contour_img: &RgbImage,
    hillshade_color_img: &RgbImage,
) -> RgbaImage {
    // Convert all to RGBA
    let gray = DynamicImage::ImageLuma8(grayscale_img.clone()).to_rgba8();
    let color = DynamicImage::ImageRgb8(color_img.clone()).to_rgba8();
    let hill_contour = DynamicImage::ImageRgb8(hillshade_contour_img.clone()).to_rgba8();
    let hill_color = DynamicImage::ImageRgb8(hillshade_color_img.clone()).to_rgba8();

    let (width, height) = gray.dimensions();
    let mut composite = RgbaImage::new(width * 2, height * 2);

    imageops::overlay(&mut composite, &gray, 0, 0);
    imageops::overlay(&mut composite, &color, width as i64, 0);
    imageops::overlay(&mut composite, &hill_contour, 0, height as i64);
    imageops::overlay(&mut composite, &hill_color, width as i64, height as i64);

    composite
}