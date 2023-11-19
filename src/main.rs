use fltk::app::event_key;
use fltk::app::event_key_down;
use fltk::enums::Color;
use fltk::enums::Event;
use fltk::enums::Key;
use fltk::prelude::WidgetExt;
use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window};
use fltk::{prelude::*, *};
use glob::*;
use std::hint::black_box;
use std::io::empty;
use std::path::*;
use std::vec;
mod key_map;
mod key_mgr;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut all_path: Vec<String> = Vec::new();
    if args.len() <= 1 {
        return;
    }
    if args[1] == "-d" { 
        if args.len() <= 2 {
            return;
        }
        all_path = get_all_path(&args[2]);
    }
    else{
        all_path.push(args[1].clone());
    }
    let app = app::App::default();
    let mut wind = Window::default().with_size(850, 850);
    wind.clone().set_color(Color::Black);
    
    match wind.label_color(){
        Color::Black => {
            println!("Bl");
        }
        _ => ()
    }
    //let mut image_vis = group::Flex::new(70,0,800,600,"");
    let mut image_N = Frame::new(25, 0, 800, 800, "");
    let mut image_P = SharedImage::load(&all_path[0]).unwrap();
    image_P.scale(800, 800, true, true);
    image_N.set_image(Some(image_P.clone()));
    wind.redraw();

    key_mgr::key_listener(image_N.clone(), image_P.clone(), wind.clone(), all_path);

    wind.end();
    
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}
fn image_set(mut image_P: image::SharedImage, mut image_N: Frame, size: f64, xp: i32, yp: i32) {
    let size: i32 = size as i32;
    image_P.scale(size, size, true, true);
    image_N.set_size(size, size);
    image_N.set_pos(xp, yp);
    image_N.set_image(Some(image_P.clone()));
    app::sleep(0.01);
}
fn get_all_path(path_in: &String) -> Vec<String> {
    let mut all_path: Vec<String> = Vec::new();
    let mut pattern = format!("{}/*.jpg", path_in);
    println!("{}",pattern);
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => all_path.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }
    pattern = format!("{}/*.png", path_in);
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => all_path.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }
    pattern = format!("{}/*.bmp", path_in);
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => all_path.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }
    all_path
}
