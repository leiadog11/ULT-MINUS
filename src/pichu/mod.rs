use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod dengekidama;
pub mod kaminari;
pub mod cloud;

const FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_BLOWN_UP: i32 = 0x200000E7;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    dengekidama::install();
    kaminari::install();
    cloud::install();
}
