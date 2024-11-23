use super::*;

pub mod acmd;
pub mod frame;
pub mod status;

pub const FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA: i32 = 0x200000E4;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}