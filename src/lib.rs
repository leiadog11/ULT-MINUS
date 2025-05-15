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
    skyline::hooks::InlineCtx,
    skyline::libc::*,
    skyline::nn::ro::LookupSymbol,
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
mod metaknight;
mod pit;
mod jack;

// GLOBAL VARIABLES
pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;
pub static mut FIGHTER_MANAGER: usize = 0;

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

// GET FINAL SMASH
unsafe extern "C" fn get_final_smash(boma: *mut BattleObjectModuleAccessor) {
    let ENTRY_ID = get_entry_id(boma);
    LookupSymbol(
        &mut FIGHTER_MANAGER,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
    );
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    smash::app::lua_bind::FighterManager::set_final(
        fighter_manager, 
        FighterEntryID(ENTRY_ID.try_into().unwrap()), 
        smash::app::FighterAvailableFinal { _address: *(smash::lib::lua_const::FighterAvailableFinal::DEFAULT) as u8 },
        0
    );
}

// GET STOCK COUNT
unsafe extern "C" fn get_stock_count(boma: *mut BattleObjectModuleAccessor) -> u32 {
    let ENTRY_ID = get_entry_id(boma);
    LookupSymbol(
        &mut FIGHTER_MANAGER,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
    );
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager, FighterEntryID(ENTRY_ID.try_into().unwrap())) as u64;
    println!("FIGHTER INFO: {}", fighter_info);
    let stock_ref = ((*((fighter_info + 8) as *const u64) + 0xd8) as *mut u32);
    println!("STOCKS: {}", stock_ref);
    return *stock_ref;
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
    metaknight::install();
    pit::install();
    jack::install();
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ganon", "gsword", false);
    smashline::update_weapon_count(*WEAPON_KIND_LUIGI_FIREBALL, 15);
    smashline::update_weapon_count(*WEAPON_KIND_PACMAN_BIGPACMAN, 4);
    smashline::update_weapon_count(*WEAPON_KIND_LINK_SWORD_BEAM, 3);
    smashline::update_weapon_count(*WEAPON_KIND_GAMEWATCH_FOOD, 15);
}