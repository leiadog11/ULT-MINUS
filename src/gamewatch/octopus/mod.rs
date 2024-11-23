use super::*;

mod attackairn;
mod acmd;
mod frame;

pub fn install() {
    attackairn::install();
    acmd::install();
    frame::install();
}