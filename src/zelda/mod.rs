use super::*;

pub mod acmd;
pub mod frame;

pub fn install() {
    acmd::install();
    frame::install();
}