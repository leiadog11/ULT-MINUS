#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;

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

pub const SITUATION_KIND:                  i32 = 0x16;
pub const FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN: i32 = 0x100000BA;
pub const FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT: i32 = 0x100000BB;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}