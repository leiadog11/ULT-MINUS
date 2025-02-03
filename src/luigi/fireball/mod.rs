use super::*;

mod acmd;
mod status;
mod frame;

static mut FIREBALL_SPEED_X: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    status::install();
    frame::install();
}