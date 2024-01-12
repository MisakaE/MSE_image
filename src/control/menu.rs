use crate::struct_command::{self, ListMsg, PosMsg, SizeMsg, MenuMsg};
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
    thread,
};

use fltk::group::Flex;
use fltk::prelude::WidgetBase;
use fltk::prelude::GroupExt;
pub fn menu_control(menu_msg_rec:Receiver<MenuMsg>){
    let mut menu_list = Flex::new(0, 0, 200, 100, None).column();
        
        let mut image_name = Frame::default().with_label("title");
        let mut image_size = Frame::default().with_label("title");
        let mut image_time = Frame::default().with_label("title");
        image_name.set_frame(fltk::enums::FrameType::FlatBox);
        image_size.set_frame(fltk::enums::FrameType::FlatBox);

        image_name.set_color(Color::from_rgb(254, 254, 254));
        image_size.set_color(Color::from_rgb(254, 254, 254));

        menu_list.fixed(&image_name, 30);
        menu_list.fixed(&image_size, 30);
        menu_list.fixed(&image_time, 30);

    menu_list.end();
    thread::spawn(move || {
        for msg in menu_msg_rec{
            image_name.set_label(&msg.name);
            image_size.set_label(&format!("{}",msg.size));
        }
    });
    
}
