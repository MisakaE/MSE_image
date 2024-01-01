use std::{sync::mpsc,env};
use fltk::{enums::Color,prelude::WidgetExt,app, frame::Frame, image::SharedImage, prelude::*, window::Window};
use glob::*;
mod key_map;
mod key_mgr;
mod struct_command;
mod control;
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
    wind.set_color(Color::Black);
    let mut image_n = Frame::new(25, 0, 800, 800, "");
    let mut image_p = SharedImage::load(&all_path[0]).unwrap();
    image_p.scale(900, 900, true, true);
    image_n.set_image(Some(image_p.clone()));
    wind.end();
    wind.make_resizable(true);
    wind.show();
    let (tx, rx) = mpsc::channel::<struct_command::Command>();
    let tx1 = tx.clone();
    let max_image = all_path.len();
    control::image(all_path, image_n, image_p, max_image as i32, rx);
    key_mgr::key_listener_keep(tx);
    key_mgr::key_listener_once(tx1);
    app.run().unwrap();
}
fn get_all_path(path_in: &String) -> Vec<String> {
    let mut all_path: Vec<String> = Vec::new();
    let mut pattern = format!("{}/*.jpg", path_in);
    println!("{}", pattern);
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
