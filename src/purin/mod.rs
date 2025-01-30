use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

static mut CHARGE_MUL: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
