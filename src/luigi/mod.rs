use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;

static mut MISFIRE_SPECIAL_N: [bool; 8] = [false; 8];
static mut MISFIRE_UP_SMASH: [bool; 8] = [false; 8];
static mut DOWN_TILT_COUNTER: [i32; 8] = [0; 8];
static mut FIREBALL_SPEED_X: [f32; 8] = [0.0; 8];
static mut NEG_ZONE: [f32; 8] = [0.0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();
}