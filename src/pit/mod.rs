use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bowarrow;
pub mod chariot;

static mut STALL_TIMER: [i32; 8] = [0; 8];
static mut SHIELD_LIFE: [i32; 8] = [0; 8];
static mut BREAK_WAIT_TIME: [i32; 8] = [0; 8];
static mut SHIELD_ON: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bowarrow::install();
    chariot::install();
}