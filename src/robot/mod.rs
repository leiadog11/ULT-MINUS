use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod beam;

static mut FIGHTER_ROBOT_INSTANCE_WORK_ID_INT_CATCH_ATTACK: i32 = 0x100000C4;
static mut FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE: i32 = 0x100000C5;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    beam::install();
}
