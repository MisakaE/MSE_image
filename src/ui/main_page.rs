use super::picture;
use fltk::{
    app,
    prelude::{GroupExt,WidgetExt},
    window,
};
use std::sync;

use crate::{
    control::{listen, listen_once},
    Msg,
};

pub fn run(list: Vec<String>) {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(1000, 1000);

    let (msg_sen, msg_rec) = sync::mpsc::channel::<Msg>();
    picture::run(&mut wind, msg_rec, list);

    wind.make_resizable(true);
    wind.end();
    wind.show();
    listen(msg_sen.clone());
    listen_once(msg_sen, &mut wind);
    app.run().unwrap();
}
