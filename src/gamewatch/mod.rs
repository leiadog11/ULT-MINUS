use super::*;

pub mod acmd;
pub mod frame;
pub mod octopus;
pub mod status;
pub mod bomb;
pub mod breath;
pub mod rescue;

pub const WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN: i32 = 0x6;
pub const FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_OCTOPUS: i32 = 0x200000E3;
pub const FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT: i32 = 0x200000E4;

pub fn install() {
    acmd::install();
    bomb::install();
    frame::install();
    octopus::install();
    breath::install();
    rescue::install();
    status::install();
}