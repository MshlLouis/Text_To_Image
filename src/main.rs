use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use image::{GenericImageView, Rgba};

fn main() {

    let path_text = "texts/file_in.txt";
    let path_img = "output/img_out.png";
    let path_output = "output/file_out.txt";

    create_image(path_text, path_img);
    read_image(path_img, path_output);

    println!("Successfully terminated program!");
}

fn create_image(path_text: &str, path_img: &str) {
    let pixel_black = image::Rgb([0u8, 0u8, 0u8]);
    let mut string_counter: usize = 0;

    let text_string = fs::read_to_string(path_text).expect("Unable to read file");
    let total_chars: f64 = text_string.len() as f64;
    let vec_chars = split_into_chunks(text_string, 3);
    let vec_chars_len = vec_chars.len();

    let img_x = (total_chars / 3f64).sqrt().ceil() as u32;
    let img_y = (total_chars / 3f64).sqrt().ceil() as u32;

    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);

    for x in 0..img_x {
        for y in 0..img_y {
            if string_counter == vec_chars_len {
                imgbuf.put_pixel(y,x,pixel_black);
            }
            else {
                let pixel = image::Rgb([*vec_chars[string_counter].get(0).unwrap(),
                    *vec_chars[string_counter].get(1).unwrap(),
                    *vec_chars[string_counter].get(2).unwrap()]);
                imgbuf.put_pixel(y,x,pixel);
                string_counter += 1;
            }
        }
    }
    imgbuf.save(path_img).unwrap();
}

fn read_image(path: &str, path_output: &str) {
    let img = image::open(path).unwrap();
    let mut all_pixels: Vec<Rgba<u8>> = vec![];

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(y, x);
            all_pixels.push(pixel);
        }
    }
    write_file(path_output, all_pixels, img.width(), img.height());
}

fn write_file(path: &str, all_pixels: Vec<Rgba<u8>>, width: u32, height: u32) {

    let path = Path::new(path);
    if path.exists() {
        fs::remove_file(path).expect("Didn't delete, file doesn't exist");
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let mut counter = 0;

    for _ in 0..width {
        for _ in 0..height {
            let pixel = all_pixels.get(counter).unwrap();

            for i in 0..3 {
                if pixel.0[i] == 0  {
                    break;
                }
                file.write((pixel.0[i] as char).to_string().as_bytes()).expect("Error");
            }
            counter += 1;
        }
    }
}

fn split_into_chunks(s: String, chunk_size: usize) -> Vec<Vec<u8>> {
    let mut chunks: Vec<Vec<u8>> = s.as_bytes()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    if let Some(last_chunk) = chunks.last_mut() {
        if last_chunk.len() < chunk_size {
            last_chunk.resize(chunk_size, 0);
        }
    }
    chunks
}