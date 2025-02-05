use super::*;

mod acmd;
mod status;

const WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_INT_SPEED_X : i32 = 0x11000004;

pub fn install() {
    acmd::install();
    status::install();
}