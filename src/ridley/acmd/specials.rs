use super::*;

//-------------------SPECIALS------------------

// UP SPECIAL
unsafe extern "C" fn ridley_specialairhi(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.9);
}

// UP SPECIAL EFFECT
unsafe extern "C" fn ridley_effect_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ridley_charge_hold"), Hash40::new("top"), 2, 0, -1, 0, 0, 0, 1, true);
    }
}

// UP SPECIAL SOUND
unsafe extern "C" fn ridley_sound_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_h01"));
    }
}

// UP SPECIAL EXPRESSION
unsafe extern "C" fn ridley_expression_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_loopattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// UP SPECIAL CHARGE
unsafe extern "C" fn ridley_specialairhicharge(agent: &mut L2CAgentBase) {
    if AURA[get_entry_id(agent.module_accessor)] { 
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 361, 10, 0, 25, 6.0, 0.0, 4.0, -18.0, Some(0.0), Some(4.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 361, 10, 0, 25, 6.0, 0.0, 10.0, -18.0, Some(0.0), Some(10.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 361, 10, 0, 25, 6.0, 0.0, 16.0, -18.0, Some(0.0), Some(16.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.2, 60, 70, 0, 65, 8.0, 0.0, 2.0, -18.0, Some(0.0), Some(2.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.2, 60, 70, 0, 65, 8.0, 0.0, 10.0, -18.0, Some(0.0), Some(10.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.2, 60, 70, 0, 65, 8.0, 0.0, 18.0, -18.0, Some(0.0), Some(18.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

// UP SPECIAL CHARGE EFFECT
unsafe extern "C" fn ridley_effect_specialairhicharge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ridley_charge"), Hash40::new("rot"), 0, 0, 21, 90, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

// UP SPECIAL CHARGE SOUND
unsafe extern "C" fn ridley_sound_specialairhicharge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_ridley_special_h02"));
    }
}

// UP SPECIAL CHARGE EXPRESSION
unsafe extern "C" fn ridley_expression_specialairhicharge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_fly"), 12, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_fly"), 12, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// SIDE SPECIAL START
unsafe extern "C" fn ridley_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 8.0, 6.0, 7.5, 7.5);
    }
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 6.0, 7.5, 5.5);
    }
    frame(agent.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 9.8, 0.0, 9.2, 26.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 5.3, 0.0, 9.2, 26.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 2, Hash40::new("top"), 8.0, 0.0, 9.2, 16.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}


// DOWN SPECIAL STAB
unsafe extern "C" fn ridley_speciallwstab(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) <= 0 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 361, 100, 20, 0, 5.1, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, -37, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_DISABLE_CRITICAL_SPECIAL_LW) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 45.0, 361, 30, 0, 50, 5.1, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -37, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 5.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.2);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_specialairhi", ridley_specialairhi, Low)
        .effect_acmd("effect_specialairhi", ridley_effect_specialairhi, Low)
        .sound_acmd("sound_specialairhi", ridley_sound_specialairhi, Low)
        .expression_acmd("expression_specialairhi", ridley_expression_specialairhi, Low)

        .game_acmd("game_specialairhicharge", ridley_specialairhicharge, Low)
        .effect_acmd("effect_specialairhicharge", ridley_effect_specialairhicharge, Low)
        .sound_acmd("sound_specialairhicharge", ridley_sound_specialairhicharge, Low)
        .expression_acmd("expression_specialairhicharge", ridley_expression_specialairhicharge, Low)

        .game_acmd("game_specialsstart", ridley_specialsstart, Low)
        .game_acmd("game_specialairsstart", ridley_specialsstart, Low)

        .game_acmd("game_speciallwstab", ridley_speciallwstab, Low)
        .game_acmd("game_specialairlwstab", ridley_speciallwstab, Low)

        .install();
}