use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

const FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2: i32 = 0x1ED;

static mut KICK: [bool; 8] = [false; 8];
static mut GUN_COOLDOWN: [i32; 8] = [0; 8];
static mut UP_B_AMOUNT: [i32; 8] = [2; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}