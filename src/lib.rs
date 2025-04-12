#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(static_mut_refs)]
#![allow(improper_ctypes_definitions)]

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
mod mario;
mod link;
mod bayonetta;
mod zelda;
mod pit;

// GLOBAL VARIABLES
pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;

// THE GREAT OPPONENT BOMA LIST
unsafe extern "C" fn get_opponent_bomas(boma: *mut BattleObjectModuleAccessor) -> Vec<*mut BattleObjectModuleAccessor> { 
    let entry_count = lua_bind::FighterManager::entry_count(singletons::FighterManager());
    let entry_count_usize = entry_count as usize;
    let mut opponent_bomas: Vec<*mut BattleObjectModuleAccessor> = Vec::with_capacity(entry_count_usize);
    let mut boma_counter = 0;
    
    for _ in 0..entry_count_usize { 
        let mut curr_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(boma_counter));
        if curr_boma != boma {
            opponent_bomas.push(sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(boma_counter)));
        }
        boma_counter += 1;
    }

    return opponent_bomas;
}

// GET ENTRY ID
unsafe extern "C" fn get_entry_id(boma: *mut BattleObjectModuleAccessor) -> usize { 
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

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
    mario::install();
    link::install();
    bayonetta::install();
    zelda::install();
    pit::install();
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ganon", "gsword", false);
    smashline::update_weapon_count(*WEAPON_KIND_LUIGI_FIREBALL, 15);
    smashline::update_weapon_count(*WEAPON_KIND_PACMAN_BIGPACMAN, 4);
    smashline::update_weapon_count(*WEAPON_KIND_LINK_SWORD_BEAM, 3);
    smashline::update_weapon_count(*WEAPON_KIND_GAMEWATCH_FOOD, 15);
}