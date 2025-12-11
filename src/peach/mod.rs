use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut STALL_TIMER: [i32; 8] = [0; 8];
static mut FORWARD_AIR_CHARGE: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
