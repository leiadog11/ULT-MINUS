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

static mut UP_B_USED : bool = false;
static mut EQUIPPED : bool = false;
static mut MIPHAS_GRACE : bool = false;
static mut DARUKS_PROTECTION : bool = false;
static mut REVALIS_GALE : bool = false;
static mut URBOSAS_FURY : bool = false;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    parasail::install();
}