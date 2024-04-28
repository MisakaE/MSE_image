use std::{cmp, error::Error, path::Path, sync::mpsc::Receiver, thread};

use crate::{Msg, MsgType, MAX_RATIO};
use fltk::{
    app::awake,
    frame,
    image::{PngImage, SharedImage},
    prelude::{ImageExt, WidgetExt},
    window::DoubleWindow,
};
use image::{codecs::png::PngEncoder, ImageEncoder};

pub fn run(parent: &mut DoubleWindow, msg_rec: Receiver<Msg>) {
    let mut image_frame = frame::Frame::default();
    //let path = "./new/DSC00902.JPG".to_string();
    let path = "./new/DSC00902.JPG".to_string();
    let mut image_body = reade_img(path).unwrap();
    image_body.scale(parent.w(), parent.h(), true, true);

    image_frame.set_pos(parent.w() / 2, parent.h() / 2);
    image_frame.set_image(Some(image_body.clone()));

    thread::spawn(move || {
        let mut size = cmp::max(image_body.h(),image_body.w());
        let mut step = 0;
        for msg in msg_rec {
            match msg.msg {
                MsgType::Move(x, y) => {
                    image_frame.set_pos(image_frame.x() + x, image_frame.y() + y);
                    awake();
                    image_frame.parent().unwrap().redraw();
                }
                MsgType::Change(_f) => (),
                MsgType::Resize(f) => {
                    if f {
                        if step < MAX_RATIO{
                            size = (size as f64 * 1.25 ) as i32;
                            step+=1;
                            image_body.scale(
                                size,
                                size,
                                true,
                                true,
                            );
                            image_frame.set_image(Some(image_body.clone()));
                            awake();
                            image_frame.parent().unwrap().redraw();
                        }
                    } else {
                        if step > -MAX_RATIO{
                            size = (size as f64 * 0.8 ) as i32;
                            step-=1;
                            image_body.scale(
                                size,
                                size,
                                true,
                                true,
                            );
                            image_frame.set_image(Some(image_body.clone()));
                            awake();
                            image_frame.parent().unwrap().redraw();
                        }
                        
                    };
                    
                }
            }
        }
    });

    parent.redraw();
}

fn reade_img<P: AsRef<Path>>(path: P) -> Result<SharedImage, Box<dyn Error>> {
    let mut f = true;
    match path.as_ref().extension() {
        Some(t) => match t.to_str().unwrap() {
            "JPG" | "jpg" | "png" | "PNG" | "BMP" | "bmp" | "SVG" | "svg" => (),
            _ => f = false,
        },
        None => return Err("File fmt empty".into()),
    }
    if f {
        let img = SharedImage::load(path)?;
        return Ok(img);
    }
    let img = image::open(path)?;
    let mut data = vec![];
    let encoder = PngEncoder::new(&mut data);
    encoder.write_image(
        img.as_bytes(),
        img.width(),
        img.height(),
        image::ColorType::Rgb8,
    )?;
    let img = PngImage::from_data(&data)?;
    let img = SharedImage::from_image(img)?;
    Ok(img)
}
