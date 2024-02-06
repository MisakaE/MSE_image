use std::thread;
use ui::mainpage::mk_main_page;

mod ui;
mod structs;
mod parser;
fn main() {
    let mainstart = thread::spawn(|| {
        mk_main_page().unwrap();
    });
    mainstart.join().unwrap();
    
    
}
