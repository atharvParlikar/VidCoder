use image::{ImageFormat, io::Reader as ImageReader, Rgb, RgbImage};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn encoder(bitmap: &Vec<[[u8; 3]; 1080]>, file_path: String) -> Result<(), std::io::Error> {
    let mut img = RgbImage::new(1920, 1080);

    for x in 0..bitmap.len() {
        for y in 0..bitmap[x].len() {
            img.put_pixel(x as u32, y as u32, Rgb(bitmap[x][y]));
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
    for x in 0..1920 {
        for y in (0..1080).step_by(3) {
           for d in img.get_pixel(x,y).0 {
                data.push(d);
            }
        }
    } 
    return Ok(data);
}

fn create_bitmap(file_path: &str) -> Vec<[[u8; 3]; 1080]> {
    let mut bitmap = vec![[[0 as u8; 3]; 1080]; 1920];
    let path = Path::new(file_path);

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open '{}': {}", path.display(), why),
    };

    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => (),
        Err(err) => panic!("error occured: {}", err),
    }

    let mut new_data: Vec<[u8; 3]> = Vec::new();

    let mut pointer = 2;

    while pointer <= data.len() - 1 {
        new_data.push([data[pointer - 2], data[pointer - 1], data[pointer]]);
        pointer += 3;
    }

    if data.len() % 3 == 2 {
        let ln = data.len();
        new_data.push([data[ln - 2], data[ln - 1], 0]);
    } else if data.len() % 3 == 1 {
        new_data.push([data[data.len() - 1], 0, 0]);
    }

    let mut x = 0;
    let mut y = 0;
    for pixel in new_data {
        for i in x..x + 3 {
            for j in y..y + 3 {
                bitmap[i][j] = pixel;
            }
        }
        if y >= 1069 {
            x += 3;
            y = 0;
        }
        y += 3;
    }

    return bitmap;
}

fn bitmap(file_path: &str) -> Result<(), std::io::Error>{
    let file = match File::open(Path::new(file_path)) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };

    return Ok(());
}

fn main() -> Result<(), std::io::Error> {
    // let bitmap = create_bitmap("./test");
    // encoder(&bitmap, "./encoded.png".to_string());
    // let decoded = decoder("./encoded.png")?;
    // File::create("./data")?.write(&decoded);
    let mut img = RgbImage::new(1920, 1080);
    for x in 0..1920 {
        img.put_pixel(x, 0, Rgb([255, 0, 0]));
    }
    img.save_with_format("./encoded.png", ImageFormat::Png);
    return Ok(());
}
