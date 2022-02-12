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
use std::sync::{Arc, Mutex};

/// z(n) = z(n-1)^2 + c
/// Returns amount of iterations to decide that z will escape to infinity
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

/// Converts a pixel in an image plane to the point in the imaginary plane
fn pixel_to_set_point(x: u32, y: u32, width: u32, height: u32, r_s: f64, r_e: f64, i_s: f64, i_e: f64) -> num::Complex<f64> {
    return num::Complex::new(
        r_s + (x as f64 / width as f64) * (r_e - r_s),
        i_s + (y as f64 / height as f64) * (i_e - i_s),
    )
}

/// Computes a gray color of the pixel that corresponds to the point in the imaginary plane based on
/// amount of iteraitons
fn compute_pixel_color(c: num::Complex<f64>, max_iter: u32, palette: u8) -> u8 {
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

    return pixel_color;
}

fn main() {
    let mut re_start: f64 = -2.5;
    let mut re_end: f64 = 1.5;
    let mut im_start: f64 = -2.0;
    let mut im_end: f64 = 2.0;

    let mut width: u32 = 7680;
    let mut height: u32 = 4320;
    let mut max_iter: u32 = 1000;
    let mut image_name: String = String::from("mandelbrot");
    let mut palette: u8 = 0;

    let matches = clap::App::new("mandelplot")
        .version("0.3.0")
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
        .arg(
            Arg::new("m_set_dimensions")
                .help("Set real and imaginary constraints (real_start,imaginary_startxreal_end,imaginary_end)")
                .takes_value(true)
                .required(false)
                .default_value("-2.5,-2.0x1.5,2.0")
                .long("m_set_dimensions")
                .short('z')
        )
        .get_matches();

    // process given options
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

    if let Some(arg_m_set_dimensions) = matches.value_of("m_set_dimensions") {
        match arg_m_set_dimensions.split_once("x") {
            Some((start, end)) => {
                match start.split_once(",") {
                    Some((real_start, imaginary_start)) => {
                        re_start = real_start.parse::<f64>().unwrap();
                        im_start = imaginary_start.parse::<f64>().unwrap();
                    }
                    None => {}
                }

                match end.split_once(",") {
                    Some((real_end, imaginary_end)) => {
                        re_end = real_end.parse::<f64>().unwrap();
                        im_end = imaginary_end.parse::<f64>().unwrap();
                    }
                    None => {}
                }
            }
            None => {}
        }
    }


    let img = Arc::new(Mutex::new(RgbImage::new(width, height)));

    // run the algorithm in a naive multi-threaded way
    const AMOUNT_OF_THREADS: usize = 24;
    let thread_work = (height as f32 / AMOUNT_OF_THREADS as f32) as u32;
    let mut threads = Vec::with_capacity(AMOUNT_OF_THREADS);

    let mut from: u32 = 0;
    for _ in 0..AMOUNT_OF_THREADS {
        let img_copy = img.clone();
        threads.push(std::thread::spawn(move || {
            for y in from..from+thread_work {
                for x in 0..width {
                    let c = pixel_to_set_point(x, y, width, height, re_start, re_end, im_start, im_end);
                    let pixel_color = compute_pixel_color(c, max_iter, palette);

                    img_copy.lock().unwrap().put_pixel(x, y, Rgb([pixel_color, pixel_color, pixel_color]));
                }
            }
        }));

        from += thread_work;
    }

    // wait for everyone to finish
    for thread in threads {
        match thread.join() {
         Ok(_) => {}
         Err(e) => { eprintln!("Error waiting for thread to finish: {:?}", e); }
        }
    }


    // save image
    match img.lock().unwrap().save_with_format(image_name + ".png", ImageFormat::Png) {
        Ok(_) => {
            println!("Saved")
        }
        Err(e) => {
            eprintln!("Could not save image: {}", e)
        }
    };
}
