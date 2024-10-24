#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
pub mod dengekidama;
pub mod kaminari;
pub mod cloud;

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

const FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_BLOWN_UP: i32 = 0x200000E7;
pub const SITUATION_KIND:                  i32 = 0x16;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    dengekidama::install();
    kaminari::install();
    cloud::install();
}
