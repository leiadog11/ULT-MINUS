use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod wariobike;

pub const FIGHTER_WARIO_GENERATE_ARTICLE_CLOUD : i32 = 0x6;
const FIGHTER_WARIO_INSTANCE_WORK_ID_INT_ATTACK_LW4 : i32 = 0x100000C3;
const WEAPON_WARIO_WARIOBIKE_STATUS_WORK_INT_BIKE_JUMP: i32 = 0x1100000D;
const FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR : i32 = 0x4E;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    wariobike::install();
}