use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;

const FIGHTER_MARIO_STATUS_KIND_SHRINK: i32 = 0x1EC;

static mut SHRUNK: [bool; 8] = [false; 8];
static mut ICEBALL: [bool; 8] = [false; 8];
static mut FORWARD_SMASH_CHARGE: [f32; 8] = [0.0; 8];
static mut FORWARD_AIR_CHARGE: [f32; 8] = [0.0; 8];
static mut STALL_TIMER: [i32; 8] = [0; 8];
static mut SHRINK_TIME: [i32; 8] = [0; 8];
static mut COIN_COUNT: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();
}