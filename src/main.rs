use fltk::app::event_key;
use fltk::app::event_key_down;
use fltk::enums::Color;
use fltk::enums::Event;
use fltk::enums::Key;
use fltk::prelude::BrowserExt;
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
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
struct commend {
    x: i32,
    y: i32,
    size:i32,
    flag:i8,
}
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
    image_p.scale(500, 500, true, true);
    image_n.set_image(Some(image_p.clone()));
    wind.end();
    wind.make_resizable(true);
    wind.show();
    let (tx, rx) = mpsc::channel::<commend>();
    let tx1 = tx.clone();
    let max_image = all_path.len();
    thread::spawn(move || {
        let mut x = 0;
        let mut y = 0;
        let mut size = 500;
        let mut now_image = 0;
        for g in rx {
            x += g.x;
            y += g.y;
            image_n.set_pos(x, y);
            if g.size != 0 {
                match g.size {
                    1 => {
                        if size <=6000 {
                            size = (size as f32 * 1.25) as i32;
                        }
                    }
                    -1 => {
                        if size >=30 {
                            size = (size as f32* 0.8) as i32;
                        }
                    }
                    _ => {}
                }
                image_p.scale(size, size, true, true);
                image_n.set_image(Some(image_p.clone()));
            }
            if g.flag!=0{
                now_image += g.flag as i32;
                if now_image == max_image as i32{
                    now_image = 0;
                }
                if now_image == -1{
                    now_image = max_image as i32 -1;
                }
                image_p = SharedImage::load(&all_path[now_image as usize]).unwrap();
                image_p.scale(size, size, true, true);
                image_n.set_image(Some(image_p.clone()));
            }
            app::awake();
            image_n.parent().unwrap().redraw();
        }
    });
    thread::spawn(move || loop {
        let mut comd = commend {
            x: 0,
            y: 0,
            size: 0,
            flag: 0,
        };
        let mut flag = false;
        if event_key_down(key_map::a) {
            comd.x -= 15;
            flag = true;
        }
        if event_key_down(key_map::d) {
            comd.x += 15;
            flag = true;
        }
        if event_key_down(key_map::w) {
            comd.y -= 15;
            flag = true;
        }
        if event_key_down(key_map::s) {
            comd.y += 15;
            flag = true;
        }
        if event_key_down(key_map::e) {
            comd.size += 1;
            flag = true;
        }
        if event_key_down(key_map::q) {
            comd.size -= 1;
            flag = true;
        }
        if flag {
            tx.send(comd).expect("field to send");
        }
        app::sleep(0.05);
    });
    thread::spawn(move||loop{
        if event_key_down(key_map::z){
            tx1.send(commend { x: 0, y: 0, size: 0, flag: -1 }).expect("field to send");
            loop {
                if !event_key_down(key_map::z){
                    break;
                }
            }
        }
        if event_key_down(key_map::c){
            tx1.send(commend { x: 0, y: 0, size: 0, flag: 1 }).expect("field to send");
            loop {
                if !event_key_down(key_map::c){
                    break;
                }
            }
        }
        app::sleep(0.05);
    });
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
