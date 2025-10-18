use super::*;

mod acmd;
mod status;

const WEAPON_PIT_CHARIOT_STATUS_KIND_BREAK: i32 = 0x1EA;

pub fn install() {
    acmd::install();
    status::install();
}