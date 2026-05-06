use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod kinopio;

static mut FORWARD_AIR_CHARGE: [f32; 8] = [0.0; 8];
static mut SLEEP_MOVE: [bool; 8] = [false; 8];
static mut CAN_CANCEL_NAIR: [bool; 8] = [false; 8];
static mut TOAD_OUT: [bool; 8] = [false; 8];

const FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN: i32 = 0x1EA;
const FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_GLIDE: i32 = 0x1EB;
const FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND: i32 = 0x1EC;

const WEAPON_PEACH_KINOPIO_STATUS_KIND_APPEAR: i32 = 0x1F1;
const WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT: i32 = 0x1F2;
const WEAPON_PEACH_KINOPIO_STATUS_KIND_PUNCH: i32 = 0x1F3;
const WEAPON_PEACH_KINOPIO_STATUS_KIND_SPRAY: i32 = 0x1F4;
const WEAPON_PEACH_KINOPIO_STATUS_KIND_AIR_SPRAY: i32 = 0x1F5;
const WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL: i32 = 0x1F6;
const WEAPON_PEACH_KINOPIO_STATUS_KIND_LAND: i32 = 0x1F7;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    kinopio::install();
}
