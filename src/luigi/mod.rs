use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;

const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N : i32 = 0x200000E4;
const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4 : i32 = 0x200000E5;
const FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW : i32 = 0x100000C2;
const WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_INT_SPEED_X : i32 = 0x11000004;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();
}



