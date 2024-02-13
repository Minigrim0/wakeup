// Constants, structs, and arrays derived from /linux/include/linux/input.h
const EV_KEY: u16 = 1;

#[derive(Debug)]
#[repr(C)]
pub struct InputEvent {
    tv_sec: isize, // from timeval struct
    tv_usec: isize, // from timeval struct
    pub type_: u16,
    pub code: u16,
    pub value: i32
}

pub fn is_key_event(type_: u16) -> bool {
    type_ == EV_KEY
}
