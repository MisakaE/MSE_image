use crate::structs::err::Err;
use fltk::image::{JpegImage, PngImage, RgbImage, SharedImage, SvgImage};
use image::{
    codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPDecoder}, open, ColorType, DynamicImage, EncodableLayout, GenericImageView, ImageBuffer, ImageDecoder, ImageEncoder
};
use regex::{Error, Regex};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read},
};
pub enum Formate {
    PNG,
    JPEG,
    WEBP,
    UNKNOW,
}
pub struct Image {
    //image_data: DynamicImage,
    formate: Formate,
    size: i32,
    date: String,
    image_path: String,
}

impl Image {
    pub fn new(path: &str) -> Result<Image, Err> {
        let image = Image {
            //image_data: open(path).unwrap(),
            formate: get_formate(path),
            size: 1,
            date: path.to_string(),
            image_path: path.to_string(),
        };
        Ok(image)
    }
    pub fn to_usable(&self) -> Result<SharedImage, Err> {
        match self.formate {
            Formate::JPEG |Formate::PNG => {
                let jpeg_image = match SharedImage::load(&self.date) {
                    Ok(ok) => ok,
                    Err(_) => return Err(Err::Errs),
                };
                Ok(jpeg_image)
            }
            _ => {
                let mut y: Vec<u8> = Vec::new();

                let buf = BufWriter::new(&mut y);
                let encoder = PngEncoder::new(buf);
                let image_data = open(self.image_path.clone().as_str()).unwrap();
                
                
                match encoder.write_image(
                    &image_data.as_bytes(),
                    image_data.dimensions().0,
                    image_data.dimensions().1,
                    image_data.color(),
                ) {
                    Ok(_) => true,
                    Err(_) => return Err(Err::Errs),
                };
                
                println!("1");
                let jpeg_image = match PngImage::from_data(y.as_bytes()) {
                    Ok(ok) => ok,
                    Err(_) => return Err(Err::Errs),
                };
                
                let image = SharedImage::from_image(jpeg_image).unwrap();
                
                Ok(image)
            }
        }
    }
}

fn get_formate(path: &str) -> Formate {
    let re = Regex::new(r"[\.][A-Z]*[a-z]*$").unwrap();
    match re.find(path).unwrap().as_str() {
        ".PNG" | ".png" => Formate::PNG,
        ".JPG" | ".jpg" | "jpeg" | "JPEG" => Formate::JPEG,
        ".webp" | ".WEBP" => Formate::WEBP,
        _ => Formate::UNKNOW,
    }
}
