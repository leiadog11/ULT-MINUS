use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod parasail;

const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START : i32 = 0x1F1;
const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE : i32 = 0x1F2;
const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP : i32 = 0x1F3;
const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING : i32 = 0x1F4;

const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START : i32 = 0x1F5;
const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE : i32 = 0x1F6;
const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP : i32 = 0x1F7;

static mut UP_B_USED: [bool; 8] = [false; 8];
static mut EQUIPPED: [bool; 8] = [false; 8];
static mut MIPHAS_GRACE: [bool; 8] = [true; 8];
static mut DARUKS_PROTECTION: [bool; 8] = [true; 8];
static mut REVALIS_GALE: [bool; 8] = [true; 8];
static mut URBOSAS_FURY: [bool; 8] = [true; 8];
static mut DOWN_TILT_COUNT: [i32; 8] = [0; 8];
static mut FLOAT_TIMER: [i32; 8] = [0; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    parasail::install();
}