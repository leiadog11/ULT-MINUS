use super::*;

pub mod status;
pub mod acmd;
pub mod frame;

pub const WEAPON_PALUTENA_REFLECTIONBOARD_INSTANCE_WORK_ID_FLAG_BREAK: i32 = 0x20000008;

pub fn install() {
    status::install();
    acmd::install();
    frame::install();
}