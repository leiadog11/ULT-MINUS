use super::*;

mod acmd;
mod frame;

static mut EXPLODED: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
}