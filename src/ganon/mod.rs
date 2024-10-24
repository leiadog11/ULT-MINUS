#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
pub mod gsword;

use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash2::*,
    smash_script::*,
    smashline::*,
    smashline::Priority::*
};

pub const FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD : i32 = 0x200000E2;
pub const FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_GROUND_CHECK : i32 = 0x200000E3;
pub const FIGHTER_GANON_GENERATE_ARTICLE_GSWORD : i32 = 0x2;
pub const WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR: i32 = 0;
pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE: i32 = 0x1ED;
pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START: i32 = 0x1EE;
pub const FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2: i32 = 0x1EF;
pub const SITUATION_KIND:                  i32 = 0x16;
pub const PREV_SITUATION_KIND:                  i32 = 0x17;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    gsword::install();
}
