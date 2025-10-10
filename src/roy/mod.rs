use super::*;

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

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}