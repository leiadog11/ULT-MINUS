use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

use smashline::Main;
use std::convert::TryInto;

pub const FIGHTER_PACMAN_GENERATE_ARTICLE_GHOST: i32 = 0x7;
pub const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_THROW_UP : i32 = 0x200000eb;
pub const SITUATION_KIND:                  i32 = 0x16;
pub const PREV_SITUATION_KIND:             i32 = 0x17;

//OPFF
unsafe extern "C" fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_THROW_UP) {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_special") {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_THROW_UP);
            }
        }
    }
}



// -------GROUND-------

// DASH ATTACK
unsafe extern "C" fn pacman_attackdash(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("pizzapacman"), 0.0, 361, 100, 12, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 7.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 2.0, 12, 15, 0, 45, 5.5, 0.0, 5.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
        for _ in 0..3 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 2.0, 12, 15, 0, 45, 5.5, 0.0, 5.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            }
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                AttackModule::clear_all(agent.module_accessor);
            }
            wait(agent.lua_state_agent, 5.0);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 4.0, 60, 65, 0, 100, 7.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// FORWARD TILT
unsafe extern "C" fn pacman_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 8.0, 120, 100, 0, 20, 4.8, 7.0, -1.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 361, 100, 0, 20, 3.6, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 8.0, 361, 100, 0, 20, 3.0, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//DAFT
unsafe extern "C" fn pacman_attacks3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 6.0, 361, 1, 1, 1, 4.8, 6.5, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 361, 1, 1, 1, 3.6, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 6.0, 361, 1, 1, 1, 3.0, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TILT
unsafe extern "C" fn pacman_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.4);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 330, 55, 0, 25, 6.0, 0.0, 3.6, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.75, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP TILT
unsafe extern "C" fn pacman_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 6.5, 105, 68, 0, 43, 6.5, 4.2, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 6.5, 93, 68, 0, 43, 5.5, 2.9, -0.5, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 6.5, 93, 68, 0, 43, 3.2, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_OFF);
    }
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// FORWARD SMASH
unsafe extern "C" fn pacman_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 16.0, 361, 97, 0, 40, 5.4, 0.0, 5.4, 0.0, Some(0.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 97, 0, 40, 3.0, 0.0, 6.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 9.0, 361, 102, 0, 40, 4.3, 0.0, 5.4, 0.0, Some(0.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_PACMAN_GENERATE_ARTICLE_GHOST, false, -1);
        //AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


// DOWN SMASH
unsafe extern "C" fn pacman_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 13.0, 29, 86, 0, 30, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("ghost2"), 13.0, 29, 86, 0, 30, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 7.0, 45, 92, 0, 40, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(1.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("ghost2"), 7.0, 45, 92, 0, 40, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(1.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 7.0, 45, 92, 0, 40, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(1.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("ghost2"), 7.0, 45, 92, 0, 40, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(1.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// ------AERIALS------

// DOWN AIR
unsafe extern "C" fn pacman_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 4.0);
    }
    frame(agent.lua_state_agent, 6.0);
    for _ in 0..3 {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 245, 100, 5, 0, 4.5, 0.0, 1.8, 0.5, Some(0.0), Some(4.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 130, 100, 22, 0, 6.2, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 100, 100, 35, 0, 6.2, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 5.0);
}
if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 270, 55, 0, 55, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
}
wait(agent.lua_state_agent, 2.0);
if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
}
frame(agent.lua_state_agent, 50.0);
if macros::is_excute(agent) {
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}

// NAIR
unsafe extern "C" fn pacman_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    if macros::is_excute(agent) {
        for _ in 0..10 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 0.6, 94, 25, 0, 10, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            wait(agent.lua_state_agent, 3.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 5.2, 60, 40, 0, 60, 10.5, 0.0, 0.7, 0.0, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// FORWARD AIR
unsafe extern "C" fn pacman_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 7.65, 60, 29, 0, 66, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 7.65, 60, 29, 0, 66, 5.0, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

// UP AIR
unsafe extern "C" fn pacman_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 70, 100, 0, 13, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 70, 100, 0, 13, 5.5, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 315, 100, 0, 13, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 315, 100, 0, 13, 5.5, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


// ------SPECIALS------

//NEUTRAL B
unsafe extern "C" fn pacman_effect_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn pacman_effect_specialnbuster(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
}
unsafe extern "C" fn pacman_effect_specialnjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.4, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
}
unsafe extern "C" fn pacman_effect_specialnsmash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.4, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
}
unsafe extern "C" fn pacman_sound_specialnsmash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_shulk_special_n05"));
    }
}
unsafe extern "C" fn pacman_sound_specialnjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_shulk_special_n01"));
    }
}
unsafe extern "C" fn pacman_sound_specialnbuster(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_shulk_special_n04"));
    }
}
unsafe extern "C" fn pacman_effect_specialnspeed(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.4, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
}
unsafe extern "C" fn pacman_sound_specialnspeed(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_shulk_special_n02"));
    }
}
unsafe extern "C" fn pacman_effect_specialnshield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_equip_end"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}
unsafe extern "C" fn pacman_sound_specialnshield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_shulk_special_n03"));
    }
}


// DOWN B
unsafe extern "C" fn hydrant_fall(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 10, 5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
}


// SIDE B
unsafe extern "C" fn pacman_specialsdash(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 50, 108, 0, 50, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            wait(agent.lua_state_agent, 6.0);
            if smash::app::stage::get_stage_id() == 0x0 {
            //Battlefield
            if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x3 {
            //FD
            if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x15B {
            //Small Battlefield
            if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x5F {
            //Smashville
            if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 140.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 140.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x6B {
            //PS2
            if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0xF2 {
            //Kalos
            if PostureModule::pos_x(agent.module_accessor) >= 165.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -165.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x101 {
            //Town and City
            if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 105.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 105.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x169 {
            //Hollow Bastion
            if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else{
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 361, 66, 0, 40, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            GroundModule::set_collidable(agent.module_accessor, true);
        }
        
}
}

// UP B
unsafe extern "C" fn pacman_specialhistart(agent: &mut L2CAgentBase) {
    WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP);
        if macros::is_excute(agent) {
	    ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE);
    }
}


// ------THROWS------

// GRAB
unsafe extern "C" fn pacman_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 4.0);
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//DASH GRAB
unsafe extern "C" fn pacman_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 4.0);
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.5, Some(0.0), Some(-2.5), Some(0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 1.5, Some(0.0), Some(-2.5), Some(1.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 2.0, Some(0.0), Some(-2.5), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}


//GRAB SOUND
unsafe extern "C" fn pacman_sound_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pacman_capture"));
    }
    frame(agent.lua_state_agent, 154.0);
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

// UP THROW
unsafe extern "C" fn pacman_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 89, 100, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 0, 22);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, -1);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_THROW_UP);
    }
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 0.842);
}

// DOWN THROW
unsafe extern "C" fn pacman_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_LEAVE_NEAR_OTTOTTO(agent, -2.5, 2.5);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 300, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..2 {
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 100, 40, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
wait(agent.lua_state_agent, 6.0);
if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 270, 100, 40, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    AttackModule::set_catch_only_all(agent.module_accessor, true, false);
}
wait(agent.lua_state_agent, 1.0);
if macros::is_excute(agent) {
    macros::CHECK_FINISH_CAMERA(agent, 0, 0);
    lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.4);
}
wait(agent.lua_state_agent, 1.0);
if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
    let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
}
}

// BACK THROW
unsafe extern "C" fn pacman_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 45, 82, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(agent, 14, 3);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.6);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// FORWARD THROW
unsafe extern "C" fn pacman_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
	    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 58, 65, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
	    macros::CHECK_FINISH_CAMERA(agent, 23, 4);
	    lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
	    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// -----------TAUNTS---------------

// SIDE TAUNT
unsafe extern "C" fn pacman_appealsr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 86.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn pacman_appealsl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 86.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TAUNT
unsafe extern "C" fn pacman_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLEEP, false.into());
    }
}
unsafe extern "C" fn pacman_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLEEP, false.into());
    }
}


//-------STATUS--------


//PRE
unsafe extern "C" fn pacman_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(), smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);

    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N) as u64, 0, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N).try_into().unwrap(), 0);
    
    return 0.into();
}

//MAIN
unsafe extern "C" fn pacman_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND { //slowing shulk in air when using neutral special
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false); 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
      }
      else { //stopping shulk motion on the ground
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
      }
      
      fighter.sub_shift_status_main(L2CValue::Ptr(pacman_specialn_main_loop as *const () as _ ))
}

//MAIN LOOP
unsafe extern "C" fn pacman_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) == true {

    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    
    if MotionModule::is_end(fighter.module_accessor) != true {
    
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
        }   
        fighter.sub_shift_status_main(L2CValue::Ptr(pacman_specialn_end as *const () as _ ));
    }
    
    if fighter.global_table[PREV_SITUATION_KIND] == *SITUATION_KIND_GROUND {
    
        if fighter.global_table[SITUATION_KIND] ==  *SITUATION_KIND_GROUND {
            if fighter.global_table[PREV_SITUATION_KIND] == *SITUATION_KIND_GROUND {
                return 0.into();
            } 
        }
    
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*FIGHTER_KINETIC_TYPE_GROUND_STOP));
    }
    else {
    
        if fighter.global_table[PREV_SITUATION_KIND] == *SITUATION_KIND_GROUND {
            return 0.into();
        } 
    
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            return 0.into();
        } 
    
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    
    return 0.into();
}

//END
unsafe extern "C" fn pacman_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_attacks3", pacman_attacks3)
        .game_acmd("game_attacks3lw", pacman_attacks3lw)
        .game_acmd("game_attacklw3", pacman_attacklw3)
        .game_acmd("game_attackhi3", pacman_attackhi3)
        .game_acmd("game_attacks4", pacman_attacks4)
        .game_acmd("game_attacklw4", pacman_attacklw4)
        .game_acmd("game_attackairlw", pacman_attackairlw)
        .game_acmd("game_attackairn", pacman_attackairn)
        .game_acmd("game_attackairf", pacman_attackairf)
        .game_acmd("game_attackairhi", pacman_attackairhi)
        .game_acmd("game_specialsdash", pacman_specialsdash)
        .game_acmd("game_specialhistart", pacman_specialhistart)
        .effect_acmd("effect_specialnstart", pacman_effect_specialnstart)
        .effect_acmd("effect_specialnjump", pacman_effect_specialnjump)
        .effect_acmd("effect_specialnbuster", pacman_effect_specialnbuster)
        .effect_acmd("effect_specialnshield", pacman_effect_specialnshield)
        .effect_acmd("effect_specialnsmash", pacman_effect_specialnsmash)
        .effect_acmd("effect_specialnspeed", pacman_effect_specialnspeed)
        .sound_acmd("sound_specialnjump", pacman_sound_specialnjump)
        .sound_acmd("sound_specialnbuster", pacman_sound_specialnbuster)
        .sound_acmd("sound_specialnshield", pacman_sound_specialnshield)
        .sound_acmd("sound_specialnsmash", pacman_sound_specialnsmash)
        .sound_acmd("sound_specialnspeed", pacman_sound_specialnspeed)
        .game_acmd("game_catch", pacman_catch)
        .game_acmd("game_catchdash", pacman_catchdash)
        .sound_acmd("sound_catch", pacman_sound_catch)
        .game_acmd("game_throwhi", pacman_throwhi)
        .game_acmd("game_throwlw", pacman_throwlw)
        .game_acmd("game_throwb", pacman_throwb)
        .game_acmd("game_throwf", pacman_throwf)
        .game_acmd("game_appealsr", pacman_appealsr)
        .game_acmd("game_appealsl", pacman_appealsl)
        .game_acmd("game_appeallwr", pacman_appeallwr)
        .game_acmd("game_appeallwl", pacman_appeallwl)
        .game_acmd("game_attackdash", pacman_attackdash)
        .on_line(Main, pacman_frame)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_end)
        .install();
    Agent::new("pacman_firehydrant")
        .game_acmd("game_fall", hydrant_fall)
        .install();
}