
use std::{clone, fs::{self, read, File}, io::BufWriter, option};

use fltk::{app::{self, awake, event_key_down, redraw, sleep}, enums::{Event, Key}, image::{JpegImage, PngImage, SharedImage}, prelude::{FltkError, GroupExt, ImageExt, WidgetBase, WidgetExt}, window::{self, Window}};
use fltk::frame::Frame;
use image::{codecs::png::PngEncoder, io::Reader, load, open, GenericImageView, ImageEncoder};

use crate::structs::image::Image;
pub fn mk_main_page() -> Result<(),FltkError>{
    let app = app::App::default();

    let para = crate::parser::get_path::parameters().unwrap();
    let mut list :Vec<String>= Vec::new();
    if para.0{
        list = crate::parser::get_path::list(para.1.as_str()).unwrap();
    }
    else {
        list.push(para.1);
    }

    let mut wind = window::Window::new(100, 100, 800, 800, None);
    let mut frame_vis =Frame::default().center_of(&wind);
    let mut image_vis = SharedImage::load(list[0].as_str()).unwrap();
    image_vis.scale(1500, 1500, true, true);

    frame_vis.set_image(Some(image_vis.clone()));
    wind.make_resizable(true);
    wind.end();
    wind.show();
    
    
    let mut i = 0;
    wind.handle(move|_,event|match event {
        Event::KeyDown =>{
            if event_key_down(Key::from_char('c')){
                i+=1;
                if i == list.len(){
                    i =0;
                }
                let image = Image::new(list[i].as_str()).unwrap();
                image_vis = image.to_usable().unwrap();
                frame_vis.set_image(Some(image_vis.clone()));
            }
            if event_key_down(Key::from_char('z')){
                if i == 0{
                    i = list.len();
                }
                i -=1;
                let image = Image::new(list[i].as_str()).unwrap();
                image_vis = image.to_usable().unwrap();
                frame_vis.set_image(Some(image_vis.clone()));
            }

            if event_key_down(Key::from_char('a')){
                frame_vis.set_pos(frame_vis.x()-20, frame_vis.y());
            }
            if event_key_down(Key::from_char('s')){
                frame_vis.set_pos(frame_vis.x(), frame_vis.y()+20);
            }
            if event_key_down(Key::from_char('d')){
                frame_vis.set_pos(frame_vis.x()+20, frame_vis.y());
            }
            if event_key_down(Key::from_char('w')){
                frame_vis.set_pos(frame_vis.x(), frame_vis.y()-20);
            }
            if event_key_down(Key::from_char('e')){
                image_vis.scale((image_vis.w() as f64 * 1.25)as i32, (image_vis.h() as f64 * 1.25)as i32, true, true);
                frame_vis.set_image(Some(image_vis.clone()));
            }
            if event_key_down(Key::from_char('q')){
                image_vis.scale((image_vis.w() as f64 * 0.8)as i32, (image_vis.h() as f64 * 0.8)as i32, true, true);
                frame_vis.set_image(Some(image_vis.clone()));
            }
            redraw();
            awake();
            true
        }
        _ => false
        
    });

    

    

    app.run()?;
    Ok(())
}