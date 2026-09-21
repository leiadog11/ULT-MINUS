use super::*;

pub mod acmd;
pub mod frame;

static mut IS_ICEBALL: bool = false;

pub fn install() {
    acmd::install();
    frame::install();
}