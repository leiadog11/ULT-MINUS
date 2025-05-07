use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bowarrow;

static mut STALL_TIMER: [i32; 8] = [0; 8];
static mut SPECIAL_LW_ACTIVE: [bool; 8] = [false; 8];
static mut SPECIAL_LW_TIMER: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bowarrow::install();
}