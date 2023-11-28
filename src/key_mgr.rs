use std::{thread, sync::mpsc::Sender};
use fltk::app::{self,event_key_down};
use crate::{key_map, struct_command::{self, Command}};
pub fn key_listener_keep(control:Sender<Command>){
    thread::spawn(move || loop {
        let mut comd = struct_command::Command {
            x: 0,
            y: 0,
            size: 0,
            flag: 0,
        };
        let mut flag = false;
        if event_key_down(key_map::A) {
            comd.x -= 15;
            flag = true;
        }
        if event_key_down(key_map::D) {
            comd.x += 15;
            flag = true;
        }
        if event_key_down(key_map::W) {
            comd.y -= 15;
            flag = true;
        }
        if event_key_down(key_map::S) {
            comd.y += 15;
            flag = true;
        }
        if event_key_down(key_map::E) {
            comd.size += 1;
            flag = true;
        }
        if event_key_down(key_map::Q) {
            comd.size -= 1;
            flag = true;
        }
        if flag {
            control.send(comd).expect("field to send");
        }
        app::sleep(0.05);
    });
}
pub fn key_listener_once(control:Sender<Command>){
    thread::spawn(move||loop{
        if event_key_down(key_map::Z){
            control.send(struct_command::Command{ x: 0, y: 0, size: 0, flag: -1 }).expect("field to send");
            loop {
                if !event_key_down(key_map::Z){
                    break;
                }
            }
        }
        if event_key_down(key_map::C){
            control.send(struct_command::Command{ x: 0, y: 0, size: 0, flag: 1 }).expect("field to send");
            loop {
                if !event_key_down(key_map::C){
                    break;
                }
            }
        }
        app::sleep(0.05);
    });
}