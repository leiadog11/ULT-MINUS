use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod phantom;

static mut STALL_TIMER: [i32; 8] = [0; 8];
static mut UP_B_USED: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    phantom::install();
}