#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
pub mod explosiveflame;
pub mod reflectionboard;
pub mod beam;

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

pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL : i32 = 0x4E;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE: i32 = 0x100000BF;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_CHARGE: i32 = 0x1EA;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_SHOOT: i32 = 0x1EB;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    explosiveflame::install();
    reflectionboard::install();
    beam::install();
}
