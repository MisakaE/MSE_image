use crate::{
    key_map,
    struct_command::{PosMsg,SizeMsg, ListMsg, self},
};
use fltk::app::{self, event_key_down};
use std::{sync::mpsc::Sender, thread};


pub fn key_listener_keep(pos_msg_sen:Sender<PosMsg>,size_msg_sen:Sender<SizeMsg>) {
    thread::spawn(move || loop {

        let mut pos_msg = PosMsg {
            x: 0,
            y: 0,
            flag:false,
        };
        let mut size_msg = SizeMsg{
            size:0,
            flag:false,
        };
        /* -------------------------------------------- */
        let mut flag = false;
        if event_key_down(key_map::A) {
            pos_msg.x -= 15;
            flag = true;
        }
        if event_key_down(key_map::D) {
            pos_msg.x += 15;
            flag = true;
        }
        if event_key_down(key_map::W) {
            pos_msg.y -= 15;
            flag = true;
        }
        if event_key_down(key_map::S) {
            pos_msg.y += 15;
            flag = true;
        }
        /* ---------------------------------------- */
        if event_key_down(key_map::E) {
            size_msg.size += 1;
            flag = true;
        }
        if event_key_down(key_map::Q) {
            size_msg.size -= 1;
            flag = true;
        }
        /* ---------------------------------------- */
        if pos_msg.x!=0 || pos_msg.y!=0 {
            pos_msg_sen.send(pos_msg).expect("field to send:pos_msg");  
        }
        if size_msg.size!=0{
            size_msg_sen.send(size_msg).expect("field to send:size_msg");
        }
        if flag {
            app::sleep(0.015);
        }
        else{
            app::sleep(0.05);
        }
        
        
    });
}

pub fn key_listener_once(list_msg: Sender<ListMsg>) {
    thread::spawn(move || loop {
        if event_key_down(key_map::Z) {
            list_msg
                .send(struct_command::ListMsg{
                    flag: -1,
                })
                .expect("field to send:list_msg");
            loop {
                if !event_key_down(key_map::Z) {
                    break;
                }
                app::sleep(0.01);
            }
        }
        if event_key_down(key_map::C) {
            list_msg
                .send(struct_command::ListMsg {
                    flag: 1,
                })
                .expect("field to send:list_msg");
            loop {
                if !event_key_down(key_map::C) {
                    break;
                }
                app::sleep(0.01);
            }
        }
        app::sleep(0.08);
    });
}
