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