#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
pub mod fireball;

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

use smashline::Main;
use std::convert::TryInto;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N : i32 = 0x200000E4;
const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4 : i32 = 0x200000E5;
const FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW : i32 = 0x100000C2;
const WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_INT_SPEED_X : i32 = 0x11000004;

//pub const SUB_STATUS2:                     i32 = 0x14;
//pub const SITUATION_KIND:                  i32 = 0x16;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    fireball::install();
}



