#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bigpacman;
pub mod firehydrant;
pub mod trampoline;

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

pub const SITUATION_KIND:                  i32 = 0x16;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH : i32 = 0x200000EC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_DOWN_SMASH : i32 = 0x200000ED;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE : i32 = 0x100000C9;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN : i32 = 0x100000CA;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN : i32 = 0x100000BD;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN : i32 = 0x100000BC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN : i32 = 0x100000BB;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN : i32 = 0x100000CB;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bigpacman::install();
    firehydrant::install();
    trampoline::install();
}