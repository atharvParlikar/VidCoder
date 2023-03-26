use image::{ImageFormat, io::Reader as ImageReader, Rgb, RgbImage};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn encoder(bitmap: &Vec<Vec<[u8; 3]>>, file_path: String) -> Result<(), std::io::Error> {
    let mut img = RgbImage::new(1920, 1080);

    for x in 0..bitmap.len() {
        for y in 0..bitmap[x].len() {
            img.put_pixel(y as u32, x as u32, Rgb(bitmap[x][y]));
        }
    }

    let path = Path::new(&file_path);
    let format = ImageFormat::Png;

    match img.save_with_format(path, format) {
        Ok(()) => (),
        Err(err) => println!("Error occured: {}", err),
    }

    return Ok(());
}

fn decoder(img_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut data: Vec<u8> = Vec::new();
    let img = ImageReader::open(img_path)?.decode().unwrap().to_rgb8();
    for i in img.pixels() {
        for d in i.0 {
            data.push(d);
        }
    }
    return Ok(data);
}

fn split_num(num: u16) -> (u8, u8) {
    let a = (num >> 8) as u8;
    let b = num as u8;
    return (a, b);
}

fn join_num(nums: (u8, u8)) -> u16 {
    let a = (nums.0 as u16) << 8;
    return a | (nums.1 as u16);
}

fn bitmap_(data: Vec<u8>) -> Result<Vec<Vec<[u8; 3]>>, std::io::Error> {
    if data.len() > (1920 * 1080 * 3) {
        panic!("The limit of bitmap function [6220794 bytes] exceeded!");
    }

    let mut data_:Vec<[u8; 3]> = Vec::new();
    let len = data.len();

    for i in (2..(len - (len % 3))).step_by(3) {
        data_.push([data[i-2], data[i-1], data[i]]);
    }

    let extra = data.len() % 3;

    match extra {
        1 => data_.push([data[len - 1], 0, 0]),
        2 => data_.push([data[len - 2], data[len - 1], 0]),
        _ => {}
    }

    let mut bitmap: Vec<Vec<[u8;3]>> = Vec::new();

    if data_.len() > (1920 * 1079) - 1 {
        println!("if block");
        let mut counter = 0;
        for _ in 0..(data_.len() / 3) {
            bitmap.push(data_[counter..(counter + 1920)].to_vec());
            counter += 1920;
        }
        bitmap.push(data_[counter..data_.len()].to_vec());
        for _ in 0..(data_.len() - counter - 2) {
            bitmap[1080 - 1].push([0 as u8; 3]);
        }
    }
    
    else {
        println!("else block");
        let mut counter = 0;
        for _ in 0..(data_.len() / 1920) {
            bitmap.push(data_[counter..(counter + 1920)].to_vec());
            counter += 1920;
        }
        println!("necessary pixels painted");
        bitmap.push(data_[counter..data_.len()].to_vec());
        for _ in 0..(1080 - (counter / 1920) - 1) {
            bitmap.push(vec![[0 as u8; 3]; 1920]);
        }
        println!("empty pixels painted");
        let last_row_num = split_num(( counter / 1920) as u16);
        let last_pixel_num = split_num((data_.len() % 1920) as u16);
        bitmap[1080 - 1][1920 - 2] = [0, last_row_num.0, last_row_num.1];
        bitmap[1080 - 1][1920 - 1] = [0, last_pixel_num.0, last_pixel_num.1];
    } 

    println!("bitmap function is safe");
    return Ok(bitmap);
}

fn bitmap(data: Vec<u8>) -> Vec<Vec<[u8; 3]>> {
    let mut pixels: Vec<[u8; 3]> = data.chunks(3)
        .map(|chunk| {
            let mut pixel = [0; 3];
            for (i, val) in chunk.iter().enumerate() {
                pixel[i] = *val;
            }
            pixel
        })
        .collect();

    let missing_pixels = 1920 * 1080 - pixels.len();
    pixels.extend(std::iter::repeat([0; 3]).take(missing_pixels));

    let mut rows: Vec<Vec<[u8; 3]>> = Vec::with_capacity(1080);
    for _ in 0..1079 {
        let mut row: Vec<[u8; 3]> = Vec::with_capacity(1920);
        for _ in 0..1920 {
            let pixel = pixels.pop().unwrap();
            row.push(pixel);
        }
        rows.push(row);
    }

    // Last row with 2 empty pixels
    let mut last_row: Vec<[u8; 3]> = Vec::with_capacity(1920);
    for _ in 0..1918 {
        let pixel = pixels.pop().unwrap();
        last_row.push(pixel);
    }
    last_row.push([0; 3]);
    last_row.push([0; 3]);
    rows.push(last_row);

    rows
}
fn main() -> Result<(), std::io::Error> {
    let mut file = File::open(Path::new("/home/atharv/dev/VidCoder/src/main.rs"))?;
    let mut data:Vec<u8> = Vec::new();
    file.read_to_end(&mut data);
    println!("{}", data.len());
    let bitmap_ = bitmap(data.clone())?;
    println!("{} {}", bitmap_.len(), bitmap_[0].len());
    encoder(&bitmap_, "encoded.png".to_string());

    // let res = decoder("encoded.png")?;
    // let mut decoded = File::create(Path::new("decoded"))?;
    // decoded.write(&res);
    println!("Done!");

    return Ok(());
}
