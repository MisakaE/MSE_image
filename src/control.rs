use std::{thread, sync::mpsc::Receiver};
use crate::struct_command;
use fltk::{app,image::SharedImage,frame::Frame,prelude::{WidgetExt,ImageExt}};

pub fn image(all_path: Vec<String>,mut image_n:Frame,mut image_p:SharedImage,max_image:i32,control:Receiver<struct_command::Command>){
    thread::spawn(move || {
        let mut x = 0;
        let mut y = 0;
        let mut size = 900;
        let mut now_image = 0;
        for g in control {
            x += g.x;
            y += g.y;
            image_n.set_pos(x, y);
            if g.size != 0 {
                match g.size {
                    1 => {
                        if size <=2000 {
                            size = (size as f32 * 1.1) as i32;
                        }
                    }
                    -1 => {
                        if size >=30 {
                            size = (size as f32* 0.92) as i32;
                        }
                    }
                    _ => {}
                }
                image_p.scale(size, size, true, true);
                image_n.set_image(Some(image_p.clone()));
            }
            if g.flag!=0{
                x=0;
                y=0;
                size=900;
                image_n.set_pos(x, y);
                now_image += g.flag as i32;
                if now_image == max_image{
                    now_image = 0;
                }
                if now_image == -1{
                    now_image = max_image-1;
                }
                image_p = SharedImage::load(&all_path[now_image as usize]).unwrap();
                image_p.scale(size, size, true, true);
                image_n.set_image(Some(image_p.clone()));
            }
            app::awake();
            image_n.parent().unwrap().redraw();
        }
    });
}