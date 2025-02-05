use super::*;

mod acmd;
mod frame;

const WEAPON_WARIO_WARIOBIKE_STATUS_WORK_INT_BIKE_JUMP: i32 = 0x1100000D;

pub fn install() {
    acmd::install();
    frame::install();
}