use fltk::{
    app, enums::Color, frame::Frame, image::SharedImage, prelude::WidgetExt, prelude::*,
    window::Window,
};
use glob::*;
use std::{
    env,
    sync::{mpsc, Arc},
};
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
    let mut image_n = Frame::new(25, 0, 800, 800, "");
    let mut image_p = SharedImage::load(&all_path[0]).unwrap();
    image_p.scale(locked_wind_main.pixel_w(), locked_wind_main.pixel_h(), true, true);
    image_n.set_image(Some(image_p.clone()));
    locked_wind_main.end();
    locked_wind_main.make_resizable(true);
    locked_wind_main.show();
    drop(locked_wind_main);
    //wind.clone().set_label("sss");

    let (tx, rx) = mpsc::channel::<struct_command::Command>();
    let tx1 = tx.clone();
    //let (windatain, windataout) = mpsc::channel::<struct_command::WindowHW>();
    //let (windata_requirein,windata_requireout) = mpsc::channel::<bool>();
    let max_image = all_path.len();
    control::image(all_path, max_image as i32, image_n, image_p, rx, wind.clone());
    key_mgr::key_listener_keep(tx);
    key_mgr::key_listener_once(tx1);
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
