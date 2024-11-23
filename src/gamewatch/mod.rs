#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod octopus;
pub mod status;
pub const WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN: i32 = 0x6;
pub const FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_OCTOPUS: i32 = 0x200000E3;

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

pub fn install() {
    acmd::install();
    frame::install();
    octopus::install();
    status::install();
}