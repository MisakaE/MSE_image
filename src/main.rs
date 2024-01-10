use fltk::{
    app, enums::Color, frame::Frame, image::SharedImage, prelude::WidgetExt,
    prelude::*, window::Window,
};
use glob::*;
use std::{
    env,
    sync::{mpsc, Arc},
};
use struct_command::{ListMsg, PosMsg, SizeMsg};
mod control;
mod key_map;
mod key_mgr;
mod struct_command;
use std::sync::Mutex;
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
    } else {
        all_path.push(args[1].clone());
    }
    let app = app::App::default();
    /*
    let mut wind = Window::default().with_size(850, 850);
    */

    let wind = Arc::new(Mutex::new(Window::default().with_size(850, 850)));
    let wind_main = wind.clone();
    let mut locked_wind_main = wind_main.lock().unwrap();
    locked_wind_main.set_color(Color::Black);

    let image_n = Arc::new(Mutex::new(Frame::new(25, 0, 800, 800, "")));

    let image_p = Arc::new(Mutex::new(SharedImage::load(&all_path[0]).unwrap()));

    let image_ps = image_p.clone();
    let image_ns = image_n.clone();
    let mut locked_image_p = image_ps.lock().unwrap();

    let mut locked_image_n = image_ns.lock().unwrap();

    locked_image_p.scale(
        locked_wind_main.pixel_w(),
        locked_wind_main.pixel_h(),
        true,
        true,
    );
    locked_image_n.set_image(Some(locked_image_p.clone()));

    locked_wind_main.end();
    locked_wind_main.make_resizable(true);
    locked_wind_main.show();
    drop(locked_wind_main);
    drop(locked_image_n);
    drop(locked_image_p);
    //wind.clone().set_label("sss");
    
    let (pos_msg_sen, pos_msg_rec) = mpsc::channel::<PosMsg>();
    let (size_msg_sen, size_msg_rec) = mpsc::channel::<SizeMsg>();
    let (list_msg_sen, list_msg_rec) = mpsc::channel::<ListMsg>();
    let max_image = all_path.len();
    control::image_control(
        pos_msg_rec, 
        list_msg_rec, 
        size_msg_rec, 
        pos_msg_sen.clone(), 
        size_msg_sen.clone(), 
        all_path, 
        max_image as i32, 
        image_n.clone(), 
        image_p.clone(), 
        wind.clone(),
    );
    
    //control::image(all_path, max_image as i32, image_n, image_p, rx, wind.clone());

    key_mgr::key_listener_keep(pos_msg_sen, size_msg_sen);
    key_mgr::key_listener_once(list_msg_sen);
    
    app.run().unwrap();
}

fn get_all_path(path_in: &String) -> Vec<String> {
    use regex::Regex;

    let mut all_path: Vec<String> = Vec::new();
    let pattern = format!("{}/*.*", path_in);
    println!("{}", pattern);
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let path_str = path.display().to_string();
                let re = Regex::new(r"[\.][a-z]*").unwrap();
                let m = re.find(path_str.as_str()).unwrap();
                match m.as_str() {
                    ".jpg" | ".png" | ".mpg" | ".xbm" | ".jpeg" => {
                        all_path.push(path.display().to_string());
                    }
                    _ => (),
                }
            }

            Err(e) => println!("{:?}", e),
        }
    }
    all_path
}
