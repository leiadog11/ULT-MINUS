use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut AURA: [bool; 8] = [false; 8];
static mut UP_B_USES: [i32; 8] = [3; 8];
static mut UP_AIR_HOLD: [i32; 8] = [0; 8];
static mut STALL_TIMER: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}