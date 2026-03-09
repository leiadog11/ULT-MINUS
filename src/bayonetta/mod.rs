use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod wickedweaveleg;

static mut RECEIVED_FINAL_SMASH: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    wickedweaveleg::install();
}