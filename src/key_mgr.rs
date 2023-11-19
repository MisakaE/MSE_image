use crate::image_set;
use crate::key_map;
use fltk::enums;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::{app, frame::Frame, image::SharedImage, window::Window};
pub fn key_listener(
    mut image_N: Frame,
    mut image_P: SharedImage,
    mut wind: Window,
    all_path: Vec<String>,
) {
    //let all_path:Vec<String> = get_all_path();
    let len = all_path.len();
    let mut size: f64 = 800.0;
    let mut xp: i32 = 25;
    let mut yp: i32 = 0;
    let mut nw = 0;
    let mut listener = Frame::default();
    listener.handle({
        let mut image_P = image_P.clone();
        let mut image_N = image_N.clone();
        let mut wind = wind.clone();
        move |_, event| match app::event_key() {
            key_map::e => {
                if app::event_key_down(enums::Key::from_char('e')) {
                    size *= 1.08;
                    image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                    wind.redraw();
                }
                true
            }
            key_map::q => {
                if app::event_key_down(enums::Key::from_char('q')) {
                    size *= 0.95;
                    image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                    wind.redraw();
                }
                true
            }
            key_map::d => {
                if app::event_key_down(enums::Key::from_char('d')) {
                    xp += 8;
                    image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                    wind.redraw();
                }
                true
            }
            key_map::a => {
                if app::event_key_down(enums::Key::from_char('a')) {
                    xp -= 8;
                    image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                    wind.redraw();
                }
                true
            }
            key_map::s => {
                if app::event_key_down(enums::Key::from_char('s')) {
                    yp += 8;
                    image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                    wind.redraw();
                }
                true
            }
            key_map::w => {
                if app::event_key_down(enums::Key::from_char('w')) {
                    yp -= 8;
                    image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                    wind.redraw();
                }
                true
            }

            _ => {
                match event {
                    enums::Event::KeyUp => {
                        if app::event_key() == enums::Key::from_char('c') {
                            nw += 1;
                            if nw >= len {
                                nw = 0;
                            }
                            image_P = SharedImage::load(&all_path[nw]).unwrap();
                        }
                        if app::event_key() == enums::Key::from_char('z') {
                            if nw == 0 {
                                nw = len;
                            }
                            nw -= 1;
                            image_P = SharedImage::load(&all_path[nw]).unwrap();
                        }
                        image_set(image_P.clone(), image_N.clone(), size, xp, yp);
                        wind.redraw();
                        ()
                    }
                    _ => (),
                }
                true
            }
        }
    });
}
