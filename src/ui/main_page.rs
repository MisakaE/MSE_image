use super::picture;
use fltk::{
    app,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window,
};
use std::sync;

use crate::{control::listen, Msg};

pub fn run() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(1000, 1000);

    let (msg_sen, msg_rec) = sync::mpsc::channel::<Msg>();
    picture::run(&mut wind,msg_rec);

    wind.end();
    wind.show();
    listen(msg_sen);
    app.run().unwrap();
}
