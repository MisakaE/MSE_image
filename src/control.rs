use crate::struct_command;
use fltk::{
    app,
    frame::Frame,
    image::SharedImage,
    prelude::{ImageExt, WidgetExt},
    window::Window, enums::Color,
};
use std::{cmp::min, sync::{mpsc::Receiver, Arc, Mutex}, thread};
pub fn image(
    all_path: Vec<String>,
    max_image: i32,
    mut image_n: Frame,
    mut image_p: SharedImage,
    control: Receiver<struct_command::Command>,
    wind: Arc<Mutex<Window>>,
) {
    let wind = wind.clone();
    thread::spawn(move || {
        let mut x = 0;
        let mut y = 0;
        let mut size = 900;
        let mut now_image = 0;
        let mut wind = wind.lock().unwrap();
        //wind.set_label("SSSS");
        wind.set_color(Color::from_rgb(33, 22, 33));
        for g in control {
            x += g.x;
            y += g.y;
            image_n.set_pos(x, y);
            if g.size != 0 {
                match g.size {
                    1 => {
                        if size <= 2000 {
                            size = (size as f32 * 1.1) as i32;
                        }
                    }
                    -1 => {
                        if size >= 30 {
                            size = (size as f32 * 0.92) as i32;
                        }
                    }
                    _ => {}
                }
                image_p.scale(size, size, true, true);
                image_n.set_image(Some(image_p.clone()));
            }
            if g.flag != 0 {
                x = 0;
                y = 0;
                image_n.set_pos(x, y);
                now_image += g.flag as i32;
                if now_image == max_image {
                    now_image = 0;
                }
                if now_image == -1 {
                    now_image = max_image - 1;
                }
                image_p = SharedImage::load(&all_path[now_image as usize]).unwrap();
                size = min(wind.pixel_h(), wind.pixel_w());

                image_p.scale(size, size, true, true);

                image_n.set_image(Some(image_p.clone()));
            }
            app::awake();
            image_n.parent().unwrap().redraw();
        }
    });
}
