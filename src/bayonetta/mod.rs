use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod wickedweaveleg;

const WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_S4: i32 = 0x1EA;
const WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_HI4: i32 = 0x1EB;

static mut RECEIVED_FINAL_SMASH: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    wickedweaveleg::install();
}