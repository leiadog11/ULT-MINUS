use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut STALL_TIMER: [i32; 8] = [0; 8];
static mut FORWARD_AIR_CHARGE: [f32; 8] = [0.0; 8];
static mut SLEEP_MOVE: [bool; 8] = [false; 8];
static mut CAN_CANCEL_NAIR: [bool; 8] = [false; 8];

const FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN: i32 = 0x1EA;
const FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND: i32 = 0x1EB;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
