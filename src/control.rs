use std::{sync::mpsc::Sender, thread};

use fltk::{app::{event_key_down, sleep}, enums::{Event, Key}, prelude::WidgetBase, window::DoubleWindow};

use crate::{Msg,MsgType};

pub fn listen(sen:Sender<Msg>){
    thread::spawn(move||{
        loop {
            if event_key_down(Key::from_char('a')){
                sen.send(Msg { msg: MsgType::Move(-20, 0) }).unwrap();
            }
            if event_key_down(Key::from_char('d')){
                sen.send(Msg { msg: MsgType::Move(20, 0) }).unwrap();
            }
            if event_key_down(Key::from_char('w')){
                sen.send(Msg { msg: MsgType::Move(0, -20) }).unwrap();
            }
            if event_key_down(Key::from_char('s')){
                sen.send(Msg { msg: MsgType::Move(0, 20) }).unwrap();
            }
            if event_key_down(Key::from_char('q')){
                sen.send(Msg { msg: MsgType::Resize(false) }).unwrap();
                sleep(0.07)
            }
            if event_key_down(Key::from_char('e')){
                sen.send(Msg { msg: MsgType::Resize(true) }).unwrap();
                sleep(0.07)
            }
            
            sleep(0.02);
        }
    });
}
pub fn listen_once(sen:Sender<Msg>,frame:&mut DoubleWindow){
    frame.handle(move|_,event|{
        match event{
            Event::KeyDown =>{
                if event_key_down(Key::from_char('z')){
                    sen.send(Msg { msg: MsgType::Change(false) }).unwrap();
                }
                if event_key_down(Key::from_char('c')){
                    sen.send(Msg { msg: MsgType::Change(true) }).unwrap();
                }
            }
            _=>()
        }
        true
    });
}