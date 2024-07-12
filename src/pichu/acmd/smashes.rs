use super::*;

//---------------SMASH ATTACKS---------------

// FORWARD SMASH - FSMASH
unsafe extern "C" fn pichu_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 1.0);
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 366, 50, 0, 10, 5.3, 0.0, 4.7, 11.4, None, None, None, 0.3, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 20, 50, 0, 10, 4.3, 0.0, 4.7, 8.0, None, None, None, 0.3, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            CancelModule::enable_cancel(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 123, 0, 68, 5.8, 0.0, 5.2, 11.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 123, 0, 68, 4.3, 0.0, 4.7, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN SMASH 
unsafe extern "C" fn pichu_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 2.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 173, 30, 0, 65, 4.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(-5.5), 1.0, 1.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        macros::FT_ADD_DAMAGE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 173, 30, 0, 65, 4.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(-5.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 50, 150, 0, 45, 6.0, 0.0, 4.5, -5.5, Some(0.0), Some(4.5), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP SMASH
unsafe extern "C" fn pichu_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 1.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 14.0, 95, 85, 0, 40, 5.4, 4.7, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 14.0, 95, 85, 0, 15, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::HIT_NODE(agent, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("mimil1"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn pichu_effect_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc3"), Hash40::new("pichu_tail_arc3"), Hash40::new("top"), 1, 5, 0, 10, -40, -110, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 3, -0.5, 32, -75, -138, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec2"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec2"), false, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//UP SMASH SOUND
unsafe extern "C" fn pichu_sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_attack06"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_smash_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_attacks4", pichu_attacks4, Low)

        .game_acmd("game_attacklw4", pichu_attacklw4, Low)

        .game_acmd("game_attackhi4", pichu_attackhi4, Low)
        .effect_acmd("effect_attackhi4", pichu_effect_attackhi4, Low)
        .sound_acmd("sound_attackhi4", pichu_sound_attackhi4, Low)
        
        .install();
}