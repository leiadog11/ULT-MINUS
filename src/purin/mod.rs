use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

const FIGHTER_PURIN_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL : i32 = 0x4E;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
