use crate::struct_command::{self, ListMsg, PosMsg, SizeMsg};
use fltk::{
    enums::Color,
    frame::Frame,
    image::SharedImage,
    macros::image,
    prelude::{ImageExt, WidgetExt},
    window::Window,
    app::{self, lock},
};
use std::{
    cmp::min,
    sync::{mpsc::{Receiver,Sender}, Arc, Mutex, MutexGuard},
    thread, fmt::Debug, ops::{Deref, DerefMut},
};
pub fn image_control(
    pos_msg_rec: Receiver<PosMsg>,
    list_msg_rec: Receiver<ListMsg>,
    size_msg_rec: Receiver<SizeMsg>,

    pos_msg_sen: Sender<PosMsg>,
    size_msg_sen: Sender<SizeMsg>,

    all_path: Vec<String>,
    max_image: i32,

    image_n: Arc<Mutex<Frame>>,
    image_p: Arc<Mutex<SharedImage>>,
    wind: Arc<Mutex<Window>>,
) {
    image_pos(pos_msg_rec, image_n.clone());
    image_size(size_msg_rec, image_n.clone(), image_p.clone());
    
    image_list(
        list_msg_rec,
        pos_msg_sen,
        size_msg_sen,
        image_p.clone(),
        wind.clone(),
        max_image,
        all_path,
    );
    
}
fn image_pos(pos_msg_rec: Receiver<PosMsg>, image_n: Arc<Mutex<Frame>>) {
    thread::spawn(move || {
        let mut x = 0;
        let mut y = 0;
        let image_ns = image_n.clone();

        for msg in pos_msg_rec {
            let mut locked_image_n = image_ns.lock().unwrap();
            if msg.flag{
                x=0;
                y=0;
            }
            x += msg.x;
            y += msg.y;
            locked_image_n.set_pos(x, y);
            locked_image_n.parent().unwrap().redraw();
            app::awake();
            drop(locked_image_n);
        }
    });
}
fn image_size(
    size_msg_rec: Receiver<SizeMsg>,
    image_n: Arc<Mutex<Frame>>,
    image_p: Arc<Mutex<SharedImage>>,
) {
    thread::spawn(move || {
        let mut size = 900;
        let image_ns = image_n.clone();
        let image_ps = image_p.clone();
        for msg in size_msg_rec {
            let mut locked_image_n = image_ns.lock().unwrap();
            let mut locked_image_p = image_ps.lock().unwrap();
            if msg.flag{
                size = msg.size;
            }
            else if msg.size == 1 {
                if size <= 2000 {
                    size = (size as f32 * 1.1) as i32;
                }
            } else {
                if size >= 30 {
                    size = (size as f32 * 0.92) as i32;
                }
            }
            locked_image_p.scale(size, size, true, true);
            locked_image_n.set_image(Some(locked_image_p.clone()));
            locked_image_n.parent().unwrap().redraw();
            app::awake();
            drop(locked_image_n);
            drop(locked_image_p);
        }
    });
}
fn image_list(
    list_msg_rec: Receiver<ListMsg>,
    pos_msg_sen: Sender<PosMsg>,
    size_msg_sen: Sender<SizeMsg>,
    image_p: Arc<Mutex<SharedImage>>,
    wind: Arc<Mutex<Window>>,
    max_image: i32,
    all_path: Vec<String>,
) {
    let image_ps = image_p.clone();
    let wind_s = wind.clone();
    let mut now_image = 0;
    thread::spawn(move || {
    
    for msg in list_msg_rec {
        let locked_wind = wind_s.lock().unwrap();
        let mut locked_image_p = image_ps.lock().unwrap();
        /*
        x = 0;
        y = 0;
        image_n.set_pos(x, y);
        */
        now_image += msg.flag as i32;
        if now_image == max_image {
            now_image = 0;
        }
        if now_image == -1 {
            now_image = max_image - 1;
        }
        locked_image_p.clone_from(&SharedImage::load(&all_path[now_image as usize]).unwrap());
        let size = min(locked_wind.pixel_h(), locked_wind.pixel_w());
        pos_msg_sen.send(PosMsg {
            x: 0,
            y: 0,
            flag: true,
        }).unwrap();
        size_msg_sen.send(SizeMsg {
            size: size,
            flag: true,
        }).unwrap();
        
        drop(locked_image_p);
        drop(locked_wind);
    }
    });
}
/*
pub fn image(
    all_path: Vec<String>,
    max_image: i32,
    mut image_n: Frame,
    mut image_p: SharedImage,
    control: Receiver<struct_command::>,
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
*/
