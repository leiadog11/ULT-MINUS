use super::*;

pub mod acmd;
pub mod frame;
pub mod octopus;
pub mod status;
pub mod bomb;
pub mod breath;
pub mod rescue;

pub const WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN: i32 = 0x6;
pub const WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACKAIRB: i32 = 0x7;
pub const WEAPON_GAMEWATCH_PARACHUTE_STATUS_KIND_ATTACKS4: i32 = 0x8;

static mut OCTOPUS: [bool; 8] = [false; 8];
static mut BOMB_OUT: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    bomb::install();
    frame::install();
    octopus::install();
    breath::install();
    rescue::install();
    status::install();
}