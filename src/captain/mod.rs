use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

pub const FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN: i32 = 0x100000BA;
pub const FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT: i32 = 0x100000BB;
pub const FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_DEC_COOLDOWN: i32 = 0x100000BC;
pub const FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_KICK: i32 = 0x200000E3;
pub const FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2: i32 = 0x1E7;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}