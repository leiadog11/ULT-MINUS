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
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

pub const SUB_STATUS2:                     i32 = 0x14;
pub const SITUATION_KIND:                  i32 = 0x16;

const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N : i32 = 0x200000E4;
const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4 : i32 = 0x200000E5;
const FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW : i32 = 0x100000C2;
const WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_INT_SPEED_X : i32 = 0x11000004;


//-------GROUND----------


//JAB 1
unsafe extern "C" fn luigi_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.4, 0.0, 6.6, 6.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.7, 0.0, 6.6, 9.1, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 180, 20, 0, 20, 2.0, 0.0, 6.6, 12.3, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 2.0, 0.0, 6.6, 12.3, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}
//JAB 2
unsafe extern "C" fn luigi_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 35, 1.5, 0.0, 5.2, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 2.8, 0.0, 5.8, 9.5, None, None, None, 1.23, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 4.0, 0.0, 6.2, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        CancelModule::enable_cancel(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

// FORWARD TILT
unsafe extern "C" fn luigi_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 0, 0, 60, 0, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 9.0, 0, 0, 60, 0, 3.8, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DAFT
unsafe extern "C" fn luigi_attacks3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 6.9, 361, 0, 1, 1, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 6.9, 361, 0, 1, 1, 3.8, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


//UP TILT
unsafe extern "C" fn luigi_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.45);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 6.0, 100, 150, 0, 8, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 6.0, 100, 150, 0, 8, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("handl"), 6.0, 100, 150, 0, 8, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
// DOWN TILT 
unsafe extern "C" fn luigi_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if  WorkModule::get_int(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW) == 3 {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 16.0, 300, 50, 0, 50, 4.8, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 16.0, 300, 50, 0, 50, 3.8, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            WorkModule::set_int(agent.module_accessor, 0, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 361, 72, 0, 32, 4.8, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 5.0, 361, 72, 0, 32, 3.8, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DASH ATTACK
unsafe extern "C" fn luigi_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 0, 10, 0, 38, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 0, 10, 0, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 0, 10, 0, 38, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 0, 10, 0, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 0, 10, 0, 38, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 0, 10, 0, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 15, 10, 0, 24, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 10, 10, 0, 48, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 128, 0, 60, 5.0, 0.0, 8.0, 12.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 128, 0, 60, 4.0, 0.0, 8.0, 5.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//FORWARD SMASH 
unsafe extern "C" fn luigi_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 6) as u64;
        if rand != 1 {
            macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 15.0, 42, 121, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_SMASH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 15.0, 42, 121, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_SMASH, *ATTACK_REGION_PUNCH);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("top"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 15.0, 42, 121, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 15.0, 42, 121, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            macros::LANDING_EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("shoulderl"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("shoulderl"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
            AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
            wait(agent.lua_state_agent, 3.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03"));    
            }
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}



//FORWARD SMASH ANGLED DOWN
unsafe extern "C" fn luigi_attacks4lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 3.0, 0, 100, 10, 0, 4.0, 0.0, 4.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 3.0, 0, 100, 10, 0, 4.0, 0.0, 4.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 15.0, 270, 116, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_SMASH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 15.0, 270, 116, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_SMASH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//UP SMASH
unsafe extern "C" fn luigi_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    let rand = smash::app::sv_math::rand(hash40("agent"), 6) as u64;
    if rand != 1 {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("snout"), *HIT_STATUS_XLU);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 14.0, 110, 102, 0, 35, 4.8, 2.8, -1.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 14.0, 110, 102, 0, 35, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4);
        }
    }
	else {
        let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        if opponent_boma == agent.module_accessor {
            opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4);
	        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 14.0, 270, 102, 0, 35, 4.8, 2.8, -1.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 14.0, 270, 102, 0, 35, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            macros::LANDING_EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
            macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03"));
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            GroundModule::set_collidable(opponent_boma, true);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//DOWN SMASH
unsafe extern "C" fn luigi_attacklw4(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 6) as u64;
    if rand != 1 {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 52, 85, 0, 40, 4.5, 0.0, 3.6, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 52, 85, 0, 40, 4.0, 0.0, 3.6, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 48, 104, 0, 40, 4.5, 0.0, 3.6, -10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 48, 104, 0, 40, 4.0, 0.0, 3.6, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 52, 85, 0, 40, 4.5, 0.0, 3.6, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 52, 85, 0, 40, 4.0, 0.0, 3.6, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::LANDING_EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("top"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
            wait(agent.lua_state_agent, 3.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03"));    
            }
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 48, 104, 0, 40, 4.5, 0.0, 3.6, -10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 48, 104, 0, 40, 4.0, 0.0, 3.6, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    } 
}   
// ------AERIALS------


//NEUTRAL AIR
unsafe extern "C" fn luigi_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 90, 90, 0, 20, 4.5, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 12.0, 90, 90, 0, 20, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 6.0, 80, 100, 0, 20, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 6.0, 80, 100, 0, 20, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// BACK AIR
unsafe extern "C" fn luigi_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 361, 100, 0, 12, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 8.0, 361, 100, 0, 12, 5.8, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 135, 0, 0, 60, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 8.0, 135, 0, 0, 60, 5.0, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);

    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


// FORWARD AIR
unsafe extern "C" fn luigi_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 361, 80, 0, 30, 5.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 8.0, 361, 80, 0, 30, 3.84, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 270, 80, 0, 30, 5.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 8.0, 270, 80, 0, 30, 3.84, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


// DOWN AIR
unsafe extern "C" fn luigi_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 270, 80, 0, 10, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 9.0, 270, 50, 0, 55, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 8.0, 270, 90, 0, 20, 7.2, 3.5, 3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 32.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 52.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn luigi_effect_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 6, 11.2, -8, 35, -30, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 4, 49.602, -59.68, 150.37, 0.35, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 2, 39.6, -59.68, 150.37, 0.43, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, -1, 59.23, -52.05, 138.77, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.3, 3.8, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true, 0.9);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 49.23, -52.05, 138.77, 0.56, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 4, 49.602, -59.68, 150.37, 0.35, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4, 2, 39.6, -59.68, 150.37, 0.43, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8, -1, 59.23, -52.05, 138.77, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 1, 49.23, -52.05, 138.77, 0.56, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}
//LUIGI UP AIR
unsafe extern "C" fn luigi_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 11.0, 55, 100, 0, 0, 4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 11.0, 55, 100, 0, 0, 4.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATK_POWER(agent, 0, 7);
        macros::ATK_POWER(agent, 1, 7);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//---------SPECIAL----------

//NEUTRAL SPECIAL
unsafe extern "C" fn luigi_specialn(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
}



//FIREBALL
unsafe extern "C" fn fireball_regular(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("fighter"), 8) as u64;
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if rand != 1 {
        WorkModule::off_flag(owner_boma, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N);
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 2.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 25, 0, 7, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 25, 0, 7, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    else {
        WorkModule::on_flag(owner_boma, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03")); 
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 88, 88, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
    }
}

//UP SPECIAL
unsafe extern "C" fn luigi_specialhi(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 6) as u64;
    if rand != 1 {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 88, 88, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 25.0, 88, 88, 0, 50, 2.5, 1.2, 6.0, 60.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::SA_SET(agent, *SITUATION_KIND_AIR);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::SET_SPEED_EX(agent, 0.0, 8.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 156, 128, 0, 102, 2.5, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
            macros::LANDING_EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
            macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03")); 
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::SA_SET(agent, *SITUATION_KIND_AIR);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//AERIAL UP SPECIAL
unsafe extern "C" fn luigi_specialairhi(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 6) as u64;
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 80, 0, 40, 2.7, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        if rand == 1 {
            macros::SET_SPEED_EX(agent, 0.0, 8.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::LANDING_EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
            macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03")); 
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
   

//AERIAL DOWN SPECIAL 
unsafe extern "C" fn luigi_specialairlw(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 8) as u64;
    if rand != 1 {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
        frame(agent.lua_state_agent, 8.0);
        execute(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, 5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, -5.5, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 2.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                macros::SET_SPEED_EX(agent, 0.0, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                wait(agent.lua_state_agent, 4.0);
            }
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.6);
        frame(agent.lua_state_agent, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 70, 130, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 89, 130, 0, 85, 6.5, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 60, 0, 85, 35.0, 0.0, 4.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        DamageModule::add_damage(agent.module_accessor, 7.0, 0);
        macros::LANDING_EFFECT(agent, Hash40::new("luigi_rocket_bomb"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("head"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
        let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
        let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_bomb_a"), Hash40::new("hip"), &dumb, &dumb, 2.0, &dumb, &dumb, false, 0, 0, 0);
        SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03"));
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE, false);
        }
    }
}


//SIDE SPECIAL WALL STICK
unsafe extern "C" fn luigi_specialswall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 13.0, 270, 80, 0, 10, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 13.0, 270, 50, 0, 55, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);

    }
}


//GRABS

//CATCH
unsafe extern "C" fn luigi_catch(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch"), false, -1.0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        macros::SEARCH(agent, 0, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
        /*
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("catch_suck"), 0.0, 1.0, false, 0.0, false, false);
        }
        */
    }
}


//SUCK
/*
unsafe extern "C" fn luigi_catchsuck(agent: &mut L2CAgentBase) {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        for _ in 0..3 {
            println!("Doin it?");
            macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            wait(agent.lua_state_agent, 1.0);
        }

        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            for _ in 0..3 {
                println!("In here?");
                macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
                macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
                wait(agent.lua_state_agent, 1.0);
            }
        }
    }

    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("catch_cut"), 0.0, 1.0, false, 0.0, false, false);
    }
}
*/

//UP THROW
unsafe extern "C" fn luigi_throwhi(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_hi"), false, -1.0);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 30, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 30, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //doubles hitbox
        //macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 6.0, 88, 55, 0, 95, 8.5, 0.0, 8.0, 10.0, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, -2, 24);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 7.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::SET_SPEED_EX(agent, 0.5, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
    }

}


//---------TAUNT------------
//DOWN TAUNT
unsafe extern "C" fn luigi_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 270, 100, 180, 0, 3.0, 0.0, -1.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//UP TAUNT
unsafe extern "C" fn luigi_appealhi(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.2);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        
    }
}

unsafe extern "C" fn luigi_sound_appealhi(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.2);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let se = SoundModule::play_se(agent.module_accessor, Hash40::new("vc_luigi_appeal01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_speed(agent.module_accessor, se as i32, 4.0); 
    }
}

//SIDE TAUNT
unsafe extern "C" fn luigi_appeals(agent: &mut L2CAgentBase) {
    println!("in this motion");
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        if true {
            macros::FT_MOTION_RATE(agent, 100.0);
        }
        
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR { 
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 270, 50, 0, 100, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
            
    }
}

//SHIELD
unsafe extern "C" fn luigi_guardon(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
        let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_timer"), Hash40::new("hip"), &dumb, &dumb, 1.2, &dumb, &dumb, false, 0, 0, 0);
        EffectModule::set_rgb(agent.module_accessor, eff.try_into().unwrap(), 0.6, 1.0, 0.6);
    }
}


//----------STATUS------------

//NEUTRAL B 


//PRE
unsafe extern "C" fn luigi_fireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

//MAIN
unsafe extern "C" fn luigi_fireball_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let facing = PostureModule::lr(weapon.module_accessor);
    let mut life = 0;
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    if !WorkModule::is_flag(owner_boma, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N) {
        life = 50;
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        speed_x = if facing == 1.0 { 1.25 } else { -1.25 };
    }
    else {
        life = 200;
        PostureModule::set_scale(weapon.module_accessor, 2.5, false);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        speed_x = if facing == 1.0 { 0.5 } else { -0.5 };
    }

    WorkModule::set_float(weapon.module_accessor, speed_x, WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_INT_SPEED_X);
    weapon.fastshift(L2CValue::Ptr(luigi_fireball_start_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn luigi_fireball_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        if opponent_boma == owner_boma {
            opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        }
    let rot_x = PostureModule::rot(weapon.module_accessor, 0);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    speed_x = WorkModule::get_float(weapon.module_accessor, WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_INT_SPEED_X);

    // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();

    PostureModule::set_rot(weapon.module_accessor, &Vector3f {x:60.0, y:60.0, z:0.0}, 0);

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        fireball_remove(weapon);
        return 0.into();
    }

    //Check hit
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        SlowModule::set_whole(weapon.module_accessor, 8, 80);
        macros::CAM_ZOOM_IN_arg5(weapon, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        macros::PLAY_SE(weapon, Hash40::new("se_common_criticalhit"));
    }

    /*
    SlowModule::clear_whole(weapon.module_accessor);
    CameraModule::reset_all(weapon.module_accessor);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
    macros::CAM_ZOOM_OUT(weapon);
    */

    // Set speed
    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    return 0.into();
}

//REMOVE
pub unsafe extern "C" fn fireball_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 1.0, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}




//SIDE B

//CHARGE - PRE
unsafe extern "C" fn luigi_specials_charge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_CHARGE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_CHARGE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_CHARGE_FLOAT,
        0
      );
      
      FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );
      
      return 0.into();
}

//CHARGE - MAIN
unsafe extern "C" fn luigi_specials_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    if !StopModule::is_stop(fighter.module_accessor) {
        luigi_specials_charge_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(luigi_specials_charge_substatus as *const () as _));
    luigi_specials_charge_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specials_charge_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_specials_charge_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_speed_mul"));
    WorkModule::add_float(fighter.module_accessor, charge_speed, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    0.into()
}

unsafe extern "C" fn luigi_specials_charge_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s_hold"),
                1.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_s_hold"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_hold"),
                1.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_s_hold"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
}

//CHARGE - MAIN LOOP
unsafe extern "C" fn luigi_specials_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
        let charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));

        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);

        //UP ANGLE
        if xpos == 0.0 && ypos > 0.0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -20.0, y: 0.0, z: 0.0}, 0);
        }
        //FORWARD ANGLE
        else if xpos > 0.0 && ypos == 0.0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
        }
        //DOWN ANGLE
        else if xpos == 0.0 && ypos < 0.0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 20.0, y: 0.0, z: 0.0}, 0);
        }



        if charge_frame <= charge {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH).into(), false.into());
            }
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            luigi_specials_charge_mot_helper(fighter);
        }
    }
    else {
        fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM).into(), false.into());
    }
    0.into()
}

//CHARGE - END
unsafe extern "C" fn luigi_specials_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CHARGE_MELEE_NO_RANDOM) != true {
        let misfire_chance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("discharge_prob"));
        let rand = smash::app::sv_math::rand(hash40("fighter"), 10) as u64;
        if rand != 9 {

        }
        else {
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE)
        }
    }
    
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
      
    return 0.into();
}

//RAM - INIT
unsafe extern "C" fn luigi_specials_ram_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::set_test_coll_stop_status(fighter.module_accessor, true);
    return 0.into();
}

//RAM - PRE
unsafe extern "C" fn luigi_specials_ram_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_FLOAT,
        0
      );
      
      FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );

    return 0.into();
}

//RAM - MAIN
unsafe extern "C" fn luigi_specials_ram_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_discharge"), 0.0, 1.0, false, 0.0, false, false);
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specials_ram_main_loop as *const () as _))
}

//RAM - MAIN LOOP
unsafe extern "C" fn luigi_specials_ram_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END).into(), false.into());
        
        return 1.into();
      }
    
      if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
          fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END).into(), false.into());
    
          return 1.into();
        }
      }
    
      if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND) {
        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
          fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END).into(), false.into());
    
          return 1.into();
        }
      }
    
      let touch = GroundModule::get_touch_flag(fighter.module_accessor);
      let lr = PostureModule::lr(fighter.module_accessor);
      
      if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK) {
    
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
    
        }
    
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER) {
          // get attack power
          // set attack power 0
        }
        else {
          if AttackModule::is_attack(fighter.module_accessor, 0, false) {
            let power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power"));
            let attack_charge = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power_charge"));
            let mut charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
            let mut charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
    
            if charge == charge_frame {
              charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE)
            }
            else {
              charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
            }
    
            charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
    
            AttackModule::set_power_mul_status(fighter.module_accessor, power * attack_charge);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);

            return 0.into();
          }
        }
        return 0.into();
      }
      else {
        let attach_side = if 0.0 <= lr {*GROUND_TOUCH_FLAG_LEFT} else { *GROUND_TOUCH_FLAG_RIGHT };
    
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
        if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_side)) {
          let wall_prob = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wall_prob"));
    
          let rand = smash::app::sv_math::rand(hash40("fighter"), 1) as u64;
        /*
          if rand != 0 {
    
          }
        */
          let wall_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wall_speed_x"));
          fighter.change_status((*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL).into(), false.into());
    
          return 0.into();
        }
        return 0.into();
    }
}

//RAM - END
unsafe extern "C" fn luigi_specials_ram_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_HIT);
    }
    
    return 0.into();
}


//OPFF
unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW) > 3 {
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }

        let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        if opponent_boma == fighter.module_accessor {
            opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        }

        let b1x = PostureModule::pos_x(fighter.module_accessor);
        let b1y = PostureModule::pos_y(fighter.module_accessor);
        let b2x = PostureModule::pos_x(opponent_boma);
        let b2y = PostureModule::pos_y(opponent_boma);
        
        // distance formula
        let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
        let d = dSquared.sqrt();
        
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD ||
           StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD_ON {
            if d < 25.0 {
                macros::SLOW_OPPONENT(fighter, 2.0, 1.0);
                DamageModule::add_damage(opponent_boma, 0.05, 0);
            }      
        }
        else {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_timer"), false, true);
        }

        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_r") ||
           MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_l") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.6, y: PostureModule::pos_y(fighter.module_accessor)});
            }

            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.6, y: PostureModule::pos_y(fighter.module_accessor)});
            }

            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
            }
    
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

//START
unsafe extern "C" fn luigi_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N);
    }
}


//-------CHECK ATTACK--------

unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}


//DOWN TILT
unsafe extern "C" fn luigi_attack_lw3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            WorkModule::add_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }
    }
    0.into()
}

//NAIR
unsafe extern "C" fn luigi_attack_airn_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
	        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
		        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
	        }
	    }
    }
    0.into()
}

//UP SMASH
unsafe extern "C" fn luigi_attack_hi4_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4) {
                GroundModule::set_collidable(opponent_boma, false);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4);
            }
	    }
    }
    0.into()
}

//UP B
unsafe extern "C" fn luigi_special_hi_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
	        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT) {
		        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_FALL.into(), false.into());
     	    }
	    }
    }
    0.into()
}


pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_attacks3lw", luigi_attacks3lw, Low)
        .game_acmd("game_attackdash", luigi_attackdash, Low)
        .game_acmd("game_attackairb", luigi_attackairb, Low)
        .game_acmd("game_attackairf", luigi_attackairf, Low)
        .game_acmd("game_attacklw3", luigi_attacklw3, Low)
        .game_acmd("game_attackhi3", luigi_attackhi3, Low)
        .game_acmd("game_attackhi4", luigi_attackhi4, Low)
        .game_acmd("game_appeallwl", luigi_appeallw, Low)
        .game_acmd("game_appeallwr", luigi_appeallw, Low)
        .game_acmd("game_appealhir", luigi_appealhi, Low)
        .game_acmd("game_appealhil", luigi_appealhi, Low)
        .sound_acmd("sound_appealhir", luigi_sound_appealhi, Low)
        .sound_acmd("sound_appealhil", luigi_sound_appealhi, Low)
        .game_acmd("game_appealsr", luigi_appeals, Low)
        .game_acmd("game_appealsl", luigi_appeals, Low)
        .game_acmd("game_attackairlw", luigi_attackairlw, Low)
        .game_acmd("game_attackairn",luigi_attackairn, Low)
        .game_acmd("game_attack11", luigi_attack11, Low)
        .game_acmd("game_attacks3", luigi_attacks3, Low)
        .game_acmd("game_attacks4", luigi_attacks4, Low)
        .game_acmd("game_attacks4lw", luigi_attacks4lw, Low)
        .game_acmd("game_attacklw4", luigi_attacklw4, Low)
        .game_acmd("game_specialswall", luigi_specialswall, Low)
        .game_acmd("game_throwhi", luigi_throwhi, Low)
        .game_acmd("game_attackairhi", luigi_attackairhi, Low)
        .game_acmd("game_specialhi", luigi_specialhi, Low)
        .game_acmd("game_specialairhi", luigi_specialairhi, Low)
        .game_acmd("game_catch", luigi_catch, Low)
        .game_acmd("game_specialairlw", luigi_specialairlw, Low)
        .game_acmd("game_attack12", luigi_attack12, Low)
        //.game_acmd("game_firebarswing1", luigi_catchsuck, Low)
        .game_acmd("game_guardon", luigi_guardon, Low)
        .game_acmd("game_specialn", luigi_specialn, Low)
        .effect_acmd("effect_attackairlw", luigi_effect_attackairlw, Low)
        .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specials_charge_pre)
        .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specials_charge_main)
        .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specials_charge_end)
        .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_init)
        .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_pre)
        .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_main)
        .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specials_ram_end)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW3, luigi_attack_lw3_check_attack_status)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, luigi_attack_airn_check_attack_status)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_HI4, luigi_attack_hi4_check_attack_status)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_HI, luigi_special_hi_check_attack_status)
        .on_line(Main, luigi_frame)
        .on_start(luigi_start)
        .install();
    Agent::new("luigi_fireball")
        .game_acmd("game_regular", fireball_regular, Low)
        .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_pre)
        .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_main)
        .install();
}



