use super::*;

mod acmd;
mod status;

const WEAPON_LUIGI_FIREBALL_IS_MISFIRE: i32 = 0;

pub fn install() {
    acmd::install();
    status::install();
}