/*
Copyright (c) 2022 Kasyanov Nikolay Alexeyevich (Unbewohnte)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use clap::Arg;
use image::ImageFormat;
use image::Rgb;
use image::RgbImage;
use num::Complex;

/// z(n) = z(n-1)^2 + c
/// Returns amount of iterations to decide that given 'c' will escape to infinity
fn mandelbrot(c: num::Complex<f64>, iterations: u32) -> Option<u32> {
    let mut z: Complex<f64> = num::Complex::new(0.0, 0.0);
    for i in 0..iterations {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    return None;
}

fn main() {
    const RE_START: f64 = -2.5;
    const RE_END: f64 = 1.5;
    const IM_START: f64 = -2.0;
    const IM_END: f64 = 2.0;

    let mut width: u32 = 7680;
    let mut height: u32 = 4320;
    let mut max_iter: u32 = 1000;
    let mut image_name: String = String::from("mandelbrot");
    let mut palette: u8 = 0;

    let matches = clap::App::new("mandelplot")
        .version("0.1.0")
        .author("Kasyanov Nikolay Alexeyevich (Unbewohnte)")
        .arg(
            Arg::new("max_iter")
                .help("Specify maximum amount of iterations to decide whether the point escapes to infinity or not")
                .takes_value(true)
                .required(false)
                .default_value("1000")
                .long("max_iter")
                .short('i')
        )
        .arg(
            Arg::new("image_dimensions")
                .help("Set image dimensions (WIDTHxHEIGHT)")
                .takes_value(true)
                .required(false)
                .default_value("7680x4320")
                .long("image_dimensions")
                .short('d')
        )
        .arg(
            Arg::new("image_name")
                .help("Set output image filename")
                .takes_value(true)
                .required(false)
                .default_value("mandelbrot")
                .long("image_name")
                .short('n')
        )
        .arg(
            Arg::new("palette")
                .help("Specify bulb color (light, dark)")
                .takes_value(true)
                .required(false)
                .default_value("light")
                .long("palette")
                .short('p')
        )
        .get_matches();

    if let Some(arg_max_iter) = matches.value_of("max_iter") {
        max_iter = arg_max_iter.parse::<u32>().unwrap();
    }

    if let Some(arg_image_dimensions) = matches.value_of("image_dimensions") {
        match arg_image_dimensions.split_once("x") {
            Some((w, h)) => {
                width = w.parse::<u32>().unwrap();
                height = h.parse::<u32>().unwrap();
            }
            None => {}
        }
    }

    if let Some(arg_image_name) = matches.value_of("image_name") {
        image_name = arg_image_name.to_string();
    }

    if let Some(arg_palette) = matches.value_of("palette") {
        match arg_palette {
            "light" => {
                palette = 0;
            }
            "dark" => {
                palette = 1;
            }
            _ => {}
        }
    }

    let mut img = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let c = num::Complex::new(
                RE_START + (x as f64 / width as f64) * (RE_END - RE_START),
                IM_START + (y as f64 / height as f64) * (IM_END - IM_START),
            );

            let pixel_color: u8;
            if let Some(iterations) = mandelbrot(c, max_iter) {
                pixel_color = ((iterations as f64 / 255.0).sin() * 255.0) as u8;
            } else {
                if palette == 0 {
                    // light
                    pixel_color = 255;
                } else {
                    // dark
                    pixel_color = 0;
                }
            }

            img.put_pixel(x, y, Rgb([pixel_color, pixel_color, pixel_color]));
        }
    }


    match img.save_with_format(image_name + ".png", ImageFormat::Png) {
        Ok(_) => {
            println!("Saved")
        }
        Err(e) => {
            eprintln!("Could not save image: {}", e)
        }
    }
}
