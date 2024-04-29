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

pub fn run(parent: &mut DoubleWindow, msg_rec: Receiver<Msg>, list: Vec<String>) {
    let mut image_frame = frame::Frame::default();
    //let path = "./new/DSC00902.JPG".to_string();
    let path = list.get(0).unwrap();
    let mut image_body = reade_img(path).unwrap();
    image_body.scale(parent.w(), parent.h(), true, true);

    image_frame.set_pos(parent.w() / 2, parent.h() / 2);
    image_frame.set_image(Some(image_body.clone()));

    thread::spawn(move || {
        let mut size = cmp::max(image_body.h(), image_body.w());
        let mut step = 0;
        let mut now = 0;
        let mut parent = image_frame.parent().unwrap();

        for msg in msg_rec {
            let mid_x = parent.w() / 2;
            let mid_y = parent.h() / 2;
            match msg.msg {
                MsgType::Move(x, y) => {
                    image_frame.set_pos(image_frame.x() + x, image_frame.y() + y);
                }
                MsgType::Change(f) => {
                    if f {
                        now += 1;
                        if now == list.len() {
                            now = 0;
                        }
                    } else {
                        if now == 0 {
                            now = list.len();
                        }
                        now -= 1;
                    }
                    let path = list.get(now).unwrap();
                    image_body = reade_img(path).unwrap();
                    image_body.scale(parent.w(), parent.h(), true, true);
                    image_frame.set_pos(parent.w() / 2, parent.h() / 2);
                    size = cmp::max(image_body.h(), image_body.w());
                    step = 0;
                    image_frame.set_image(Some(image_body.clone()));
                }
                MsgType::Resize(f) => {
                    if f {
                        if step < MAX_RATIO {
                            size = (size as f64 * 1.25) as i32;
                            step += 1;
                            image_body.scale(size, size, true, true);

                            image_frame.set_image(Some(image_body.clone()));
                            let x = ((image_frame.x() - mid_x) as f64 * 0.25) as i32;
                            let y = ((image_frame.y() - mid_y) as f64 * 0.25) as i32;
                            image_frame.set_pos(image_frame.x() + x, image_frame.y() + y);
                        }
                    } else {
                        if step > -MAX_RATIO {
                            size = (size as f64 * 0.8) as i32;
                            step -= 1;
                            image_body.scale(size, size, true, true);
                            image_frame.set_image(Some(image_body.clone()));
                            let x = ((image_frame.x() - mid_x) as f64 * -0.2) as i32;
                            let y = ((image_frame.y() - mid_y) as f64 * -0.2) as i32;
                            image_frame.set_pos(image_frame.x() + x, image_frame.y() + y);
                        }
                    };
                }
            }

            if image_body.h() < parent.h() {
                image_frame.set_pos(image_frame.x(), mid_y);
            } else {
                if image_frame.y() >= image_body.h() / 2 {
                    image_frame.set_pos(image_frame.x(), image_body.h() / 2);
                }
                if image_frame.y() <= parent.h() - image_body.h() / 2 {
                    image_frame.set_pos(image_frame.x(), parent.h() - image_body.h() / 2);
                }
            }
            if image_body.w() < parent.w() {
                image_frame.set_pos(mid_x, image_frame.y());
            } else {
                if image_frame.x() >= image_body.w() / 2 {
                    image_frame.set_pos(image_body.w() / 2, image_frame.y());
                }

                if image_frame.x() <= parent.w() - image_body.w() / 2 {
                    image_frame.set_pos(parent.w() - image_body.w() / 2, image_frame.y());
                }
            }

            awake();
            parent.redraw();
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
