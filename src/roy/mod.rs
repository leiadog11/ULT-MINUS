use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

const FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_ROLL: i32 = 0x1EA;
const FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_DIVE: i32 = 0x1EB;
const FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_LANDING: i32 = 0x1EC;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}