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
const FIGHTER_PURIN_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL : i32 = 0x4E;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
