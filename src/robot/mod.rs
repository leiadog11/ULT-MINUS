#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
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

use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

static mut FIGHTER_ROBOT_INSTANCE_WORK_ID_INT_CATCH_ATTACK: i32 = 0x100000C4;
static mut FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE: i32 = 0x100000C5;
pub const SITUATION_KIND:                  i32 = 0x16;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    beam::install();
}
