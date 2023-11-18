use std::collections::btree_map::Entry;
use std::path::*;
use std::vec;

use fltk::app::event_key;
use glob::*;
use fltk::{prelude::*, *};
use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window};
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(850, 650);

    //let mut image_vis = group::Flex::new(70,0,800,600,"");
    let mut image_N = Frame::new(25,0,800,600,"");
    let mut image_P = SharedImage::load("new/acg.gy_01.jpg").unwrap();
    
    image_P.scale(800, 600, true, true);
    //image_N.set_image(Some(image_P));
    let mut size:i32 = 500;
    let mut xp:i32 = 0;
    let mut yp:i32 = 0;
    let mut nw = 0;
    let all_path:Vec<String> = get_all_path();
    let len = all_path.len();
    image_N.handle({
    let mut image_P=image_P.clone();
    let mut image_N = image_N.clone();
    let mut wind = wind.clone();
        move |_, event |
        match event {
        enums::Event::NoEvent => {
            if app::event_key_down(enums::Key::from_char('e'))  {
                size += 40;
                image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                wind.redraw();
            }
            if app::event_key_down(enums::Key::from_char('q'))  {
                size-=40;
                image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                wind.redraw();
            }
            if app::event_key_down(enums::Key::from_char('d'))  {
                xp+=8;
                image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                wind.redraw();
            }
            if app::event_key_down(enums::Key::from_char('a'))  {
                xp-=8;
                image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                wind.redraw();
            }
            if app::event_key_down(enums::Key::from_char('s'))  {
                yp+=8;
                image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                wind.redraw();
            }
            if app::event_key_down(enums::Key::from_char('w')){
                yp-=8;
                image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                wind.redraw();
            }
            wind.redraw();
            app::sleep(0.05);
            true
        }
        enums::Event::KeyUp =>{
            if app::event_key()==enums::Key::from_char('c'){
                nw+=1;
                if nw >= len{nw = 0;}
                image_P = SharedImage::load(&all_path[nw]).unwrap();
            }
            if app::event_key()==enums::Key::from_char('z'){
                if nw == 0 {nw = len;}
                nw-=1;
                
                image_P = SharedImage::load(&all_path[nw]).unwrap();
            }
            image_set(image_P.clone(), image_N.clone(), size, xp, yp);
            wind.redraw();
            true
        }
        _ => false
    }});
    println!("{}",len);
    wind.end();
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}
fn image_set(mut image_P:image::SharedImage,mut image_N:Frame,size:i32,xp:i32,yp:i32){
    image_P.scale(size, size, true, true);
    image_N.set_size(size, size);
    image_N.set_pos(xp, yp);
    image_N.set_image(Some(image_P.clone()));
    image_N.redraw();
}
fn get_all_path() -> Vec<String>{
    let mut all_path:Vec<String> = Vec::new();
    for entry in glob("new/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => all_path.push(path.display().to_string()),
        Err(e) => println!("{:?}", e),
    }}
    all_path
}
