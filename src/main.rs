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

    let mut flosses: Vec<Floss> = serde_json::from_str(dmc_json).unwrap();

    if let Some(filename) = filename_opt {
        let img = image::open(filename).unwrap();

        let mut all_colors = BTreeSet::new();

        for pixel in img.pixels() {
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

            flosses.sort_by(|a, b| {
                let a_color = RGBColor {
                    r: a.r as f64,
                    g: a.g as f64,
                    b: a.b as f64,
                };

                let b_color = RGBColor {
                    r: b.r as f64,
                    g: b.g as f64,
                    b: b.b as f64,
                };

                // let a_dist = pixel_color.distance(&a_color);
                // let b_dist = pixel_color.distance(&b_color);
                let a_dist = (pixel_color.r - a_color.r).powf(2.0)
                    + (pixel_color.g - a_color.g).powf(2.0)
                    + (pixel_color.b - a_color.b).powf(2.0);

                let b_dist = (pixel_color.r - b_color.r).powf(2.0)
                    + (pixel_color.g - b_color.g).powf(2.0)
                    + (pixel_color.b - b_color.b).powf(2.0);

                a_dist.partial_cmp(&b_dist).unwrap()
            });

            // for floss in flosses.iter() {
            //     let floss_color = RGBColor {
            //         r: floss.r as f64,
            //         g: floss.g as f64,
            //         b: floss.b as f64,
            //     };

            //     let distance = (pixel_color.r - floss_color.r).powf(2.0)
            //         + (pixel_color.g - floss_color.g).powf(2.0)
            //         + (pixel_color.b - floss_color.b).powf(2.0);

            //     // pixel_color.distance(&floss_color);

            //     if distance < min_distance {
            //         floss_for_pixel = floss.clone();
            //         min_distance = distance;
            //     }
            // }

            let terminal_pixel_color = crossterm::Colored::Bg(crossterm::Color::Rgb {
                r: pixel_color.r as u8,
                g: pixel_color.g as u8,
                b: pixel_color.b as u8,
            });

            let terminal_black = crossterm::Colored::Bg(crossterm::Color::Rgb { r: 0, g: 0, b: 0 });

            println!(
                "{}     {} {}     {} {}     {} {}     {} {}     {}",
                terminal_pixel_color,
                terminal_black,
                terminal_pixel_color,
                terminal_black,
                terminal_pixel_color,
                terminal_black,
                terminal_pixel_color,
                terminal_black,
                terminal_pixel_color,
                terminal_black,
            );

            let terminal_floss_color = crossterm::Colored::Bg(crossterm::Color::Rgb {
                r: floss_for_pixel.r as u8,
                g: floss_for_pixel.g as u8,
                b: floss_for_pixel.b as u8,
            });

            let toBg = |color: &Floss| {
                crossterm::Colored::Bg(crossterm::Color::Rgb {
                    r: color.r as u8,
                    g: color.g as u8,
                    b: color.b as u8,
                })
            };

            println!(
                "{}{:5}{} {}{:5}{} {}{:5}{} {}{:5}{} {}{:5}{}",
                toBg(&flosses[0]),
                flosses[0].floss,
                terminal_black,
                toBg(&flosses[1]),
                flosses[1].floss,
                terminal_black,
                toBg(&flosses[2]),
                flosses[2].floss,
                terminal_black,
                toBg(&flosses[3]),
                flosses[3].floss,
                terminal_black,
                toBg(&flosses[4]),
                flosses[4].floss,
                terminal_black,
            );

            println!("");
        }
    } else {
        println!("No filename provided");
    }
}
