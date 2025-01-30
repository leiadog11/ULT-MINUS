use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod beam;

static mut PUMMEL_AMOUNT: [i32; 8] = [0; 8];
static mut GYRO_LIFE: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    beam::install();
}
