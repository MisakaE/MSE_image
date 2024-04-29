use app::dir_open::parse;

pub mod app;
pub mod ui;
pub mod control;
const MAX_RATIO:i32 = 8;
enum MsgType {
    Move(i32,i32),
    Resize(bool),
    Change(bool),
}

pub struct Msg{
    msg:MsgType
}
pub fn run(){
    let path = parse().unwrap();
    let list = app::dir_open::read_list(path).unwrap();
    ui::main_page::run(list);
    
}