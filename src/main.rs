use image::{Rgb, RgbImage};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let _ = read_random();
}

fn read_random() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("/dev/random")?;
    let mut buffer = [0; 256 * 32];

    // Read file into vector.
    f.read_exact(&mut buffer[..])?;

    let mut img = RgbImage::new(256, 256);

    for x in 0..256 {
        for y in 0..256 {
            let buffer_index = x * 32 + y / 8;
            println!("{buffer_index}");
            let number = buffer[buffer_index];
            println!("{number}");
            let bit_index = y % 8;
            let bit = (number & (1 << bit_index)) != 0;
            println!("{bit}");
            let color = bit as u8 * 255;

            img.put_pixel(x as u32, y as u32, Rgb([color, color, color]));
        }
    }
    img.save("test.png").unwrap();

    Ok(())
}
