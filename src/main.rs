use scarlet::color::Color;
use scarlet::color::RGBColor;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashSet;
// use serde_json::Result;

use image::GenericImageView;

#[derive(Clone, Serialize, Deserialize)]
struct Floss {
    floss: String,
    description: String,
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let filename_opt = std::env::args().nth(1);

    let dmc_json = include_str!("rgb-dmc.json");

    let flosses: Vec<Floss> = serde_json::from_str(dmc_json).unwrap();

    if let Some(filename) = filename_opt {
        let img = image::open(filename).unwrap();

        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        let mut all_colors = BTreeSet::new();

        for pixel in img.pixels() {
            println!("{:?}", pixel);
            all_colors.insert(PixelColor {
                r: pixel.2.data[0],
                g: pixel.2.data[1],
                b: pixel.2.data[2],
            });
        }

        for color in all_colors {
            let mut min_distance = 100000000.0;
            let mut floss_for_pixel = flosses[0].clone();

            let pixel_color = RGBColor {
                r: color.r as f64,
                g: color.g as f64,
                b: color.b as f64,
            };

            for floss in flosses.iter() {
                let floss_color = RGBColor {
                    r: floss.r as f64,
                    g: floss.g as f64,
                    b: floss.b as f64,
                };

                let distance = (pixel_color.r - floss_color.r).powf(2.0)
                    + (pixel_color.g - floss_color.g).powf(2.0)
                    + (pixel_color.b - floss_color.b).powf(2.0);
                // pixel_color.distance(&floss_color);
                if distance < min_distance {
                    floss_for_pixel = floss.clone();
                    min_distance = distance;
                }
            }

            println!(
                "{} Pixel: {:?} - {} Floss: {:?}",
                crossterm::Colored::Fg(crossterm::Color::Rgb {
                    r: pixel_color.r as u8,
                    g: pixel_color.g as u8,
                    b: pixel_color.b as u8
                }),
                pixel_color,
                crossterm::Colored::Fg(crossterm::Color::Rgb {
                    r: floss_for_pixel.r as u8,
                    g: floss_for_pixel.g as u8,
                    b: floss_for_pixel.b as u8
                }),
                floss_for_pixel.floss
            );
        }
    } else {
        println!("No filename provided");
    }
}
