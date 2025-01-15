use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod parasail;

pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START : i32 = 0x1F1;
pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE : i32 = 0x1F2;
pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP : i32 = 0x1F3;
pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP : i32 = 0x1F4;
pub const FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING : i32 = 0x1F5;

pub const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START : i32 = 0x1F6;
pub const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE : i32 = 0x1F7;
pub const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP : i32 = 0x1F8;
pub const WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP : i32 = 0x1F9;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    parasail::install();
}