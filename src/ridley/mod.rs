use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut AURA: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}