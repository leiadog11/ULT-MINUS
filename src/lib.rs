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
mod roy;

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

// REMOVE FINAL SMASH
unsafe extern "C" fn remove_final_smash(boma: *mut BattleObjectModuleAccessor) {
    LookupSymbol(
        &mut FIGHTER_MANAGER,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
    );
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS);
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE);
    smash::app::lua_bind::FighterManager::set_visible_finalbg(fighter_manager, false);
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
    let stock_ref = ((*((fighter_info + 8) as *const u64) + 0xd8) as *mut u32);
    return *stock_ref;
}

// ALL ATTACKS CHECK
unsafe extern "C" fn is_attacking(boma: *mut BattleObjectModuleAccessor) -> bool {
    let motion_kind = MotionModule::motion_kind(boma);

    if motion_kind == hash40("attack_air_b") || motion_kind == hash40("attack_air_f") || motion_kind == hash40("attack_air_n") ||  motion_kind == hash40("attack_air_lw") || 
        motion_kind == hash40("attack_air_hi") || motion_kind == hash40("attack_11") || motion_kind == hash40("attack_12") || motion_kind == hash40("attack_13") ||
        motion_kind == hash40("attack_dash") || motion_kind == hash40("catch_attack") || motion_kind == hash40("cliff_attack") || motion_kind == hash40("slip_attack") ||
        motion_kind == hash40("throw_b") || motion_kind == hash40("throw_hi") || motion_kind == hash40("throw_f") || motion_kind == hash40("throw_lw") ||
        motion_kind == hash40("attack_s3_s") || motion_kind == hash40("attack_s3_hi") || motion_kind == hash40("attack_s3_lw") || motion_kind == hash40("attack_lw3") || 
        motion_kind == hash40("attack_hi3") || motion_kind == hash40("attack_s4_s") || motion_kind == hash40("attack_s4_hi") || motion_kind == hash40("attack_s4_lw") || 
        motion_kind == hash40("attack_lw4") || motion_kind == hash40("attack_hi4") || motion_kind == hash40("special_s") || motion_kind == hash40("special_n") {
            return true;
        }
    else {
        return false;
    }
}

// --------- CANCEL FUNCTIONS --------

// CANCEL WITH JUMP
unsafe extern "C" fn cancel_with_jump(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH DASH
unsafe extern "C" fn cancel_with_dash(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let xpos = ControlModule::get_stick_x(boma);
    if (xpos > 0.5 || xpos < -0.5) && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH SHIELD
unsafe extern "C" fn cancel_with_shield(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH GRAB
unsafe extern "C" fn cancel_with_grab(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH JAB
unsafe extern "C" fn cancel_with_jab(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let ypos = ControlModule::get_stick_y(boma);
    let xpos = ControlModule::get_stick_x(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ypos == 0.0 && xpos == 0.0 && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH ATTACK S3
unsafe extern "C" fn cancel_with_attacks3(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let xpos = ControlModule::get_stick_x(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && (xpos > 0.5 || xpos < -0.5) && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH ATTACK LW3
unsafe extern "C" fn cancel_with_attacklw3(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let ypos = ControlModule::get_stick_y(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ypos < -0.5 && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH ATTACK HI3
unsafe extern "C" fn cancel_with_attackhi3(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let ypos = ControlModule::get_stick_y(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ypos > 0.5 && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH SPECIAL N
unsafe extern "C" fn cancel_with_specialn(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let ypos = ControlModule::get_stick_y(boma);
    let xpos = ControlModule::get_stick_x(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ypos == 0.0 && xpos == 0.0 && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH SPECIAL S
unsafe extern "C" fn cancel_with_specials(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let xpos = ControlModule::get_stick_x(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && (xpos > 0.5 || xpos < -0.5) && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH SPECIAL HI
unsafe extern "C" fn cancel_with_specialhi(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let ypos = ControlModule::get_stick_y(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ypos > 0.5 && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
}

// CANCEL WITH SPECIAL LW
unsafe extern "C" fn cancel_with_speciallw(boma: *mut BattleObjectModuleAccessor, cancel_frame: f32) {
    let frame = MotionModule::frame(boma);
    let ypos = ControlModule::get_stick_y(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ypos < -0.5 && frame >= cancel_frame {
        CancelModule::enable_cancel(boma);
    }
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
    roy::install();
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ganon", "gsword", false);
    smashline::update_weapon_count(*WEAPON_KIND_LUIGI_FIREBALL, 15);
    smashline::update_weapon_count(*WEAPON_KIND_RIDLEY_BREATH, 30);
    smashline::update_weapon_count(*WEAPON_KIND_PACMAN_BIGPACMAN, 4);
    smashline::update_weapon_count(*WEAPON_KIND_LINK_SWORD_BEAM, 3);
    smashline::update_weapon_count(*WEAPON_KIND_LINK_BOOMERANG, 2);
    smashline::update_weapon_count(*WEAPON_KIND_GAMEWATCH_FOOD, 15);
    smashline::update_weapon_count(*WEAPON_KIND_BAYONETTA_WICKEDWEAVELEG, 2);
}