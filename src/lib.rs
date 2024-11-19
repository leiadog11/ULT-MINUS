#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

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

mod pacman;
mod luigi;
mod robot;
mod common;
mod wario;
mod ganon;
mod purin;
mod falco;
mod pichu;
mod palutena;
mod captain;
mod ridley;
mod gamewatch;

#[skyline::main(name = "ult_minus")]
pub fn main() {
    pacman::install();
    luigi::install();
    robot::install();
    wario::install();
    ganon::install();
    common::install();
    purin::install();
    falco::install();
    pichu::install();
    palutena::install();
    captain::install();
    ridley::install();
    gamewatch::install();
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ganon", "gsword", false);
    smashline::update_weapon_count(*WEAPON_KIND_LUIGI_FIREBALL, 15);
    smashline::update_weapon_count(*WEAPON_KIND_PACMAN_BIGPACMAN, 4);
}