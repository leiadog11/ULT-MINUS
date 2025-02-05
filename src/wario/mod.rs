use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod wariobike;

static mut DOWN_SMASH_AMOUNT: [i32; 8] = [0; 8];
static mut WECTOR: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    wariobike::install();
}