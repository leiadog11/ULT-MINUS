use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;

pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL : i32 = 0x200000F1;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_SMASH_CHARGE : i32 = 0x4E;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE : i32 = 0x4F;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();
}