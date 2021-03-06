extern crate num;
extern crate image;

use image::{ColorType, codecs::png::PngEncoder};
use num::Complex;

use std::{fs::File, str::FromStr};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.34 -1,0.20", args[0]);

        std::process::exit(1);
    }

    let bounds: (usize, usize) = parse_pair(&args[2], 'x').expect("error parsing image dimensions.");
    let upper_left: Complex<f64> = parse_complex(&args[3]).expect("error parsing upper left corner.");
    let lower_right: Complex<f64> = parse_complex(&args[4]).expect("error parsing lower right corner.");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds);
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point: Complex<f64> = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = escape_time(point, 255)
                .map(|count| 255 - count as u8)
                .unwrap_or(0);
        }
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let width: f64 = lower_right.re - upper_left.re;
    let height: f64 = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) {
    let output: File = File::create(filename).unwrap();
    let encoder: PngEncoder<File> = PngEncoder::new(output);

    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8).unwrap();
}