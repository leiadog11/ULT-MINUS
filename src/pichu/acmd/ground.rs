use super::*;

//---------------GROUND MOVES-----------------

// JAB
unsafe extern "C" fn pichu_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.1);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 0, 60, 0, 10, 3.0, 0.0, 3.0, 6.0, None, None, None, 1.0, 2.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 0, 60, 0, 20, 3.0, 0.0, 3.0, 10.0, None, None, None, 1.0, 2.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

// JAB EFFECT
unsafe extern "C" fn pichu_effect_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 3.5, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.4);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec2"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec2"), false, false);
    }
}

// JAB SOUND
unsafe extern "C" fn pichu_sound_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_swing_s"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
}

// FTILT - FORWARD TILT
unsafe extern "C" fn pichu_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.4);
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 8.0, 361, 130, 0, 40, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(-4.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("footr"), 8.0, 361, 130, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-3.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DAFT
unsafe extern "C" fn pichu_attacks3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.1);
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 8.0, 361, 1, 1, 1, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(-4.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("footr"), 8.0, 361, 1, 1, 1, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-3.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TILT
unsafe extern "C" fn pichu_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.5);
        macros::ATTACK(agent, 0, 0, Hash40::new("tail1"), 6.0, 80, 30, 0, 30, 3.2, 1.7, 0.4, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("tail3"), 6.0, 100, 30, 0, 30, 4.0, -0.6, 0.8, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TILT EFFECT
unsafe extern "C" fn pichu_effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc2"), Hash40::new("pichu_tail_arc2"), Hash40::new("top"), 0, 5.5, 3, -13, 0, -160, 0.9, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("tail3"), 2, 0, 3, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec2"), Hash40::new("tail1"), 2, 0, 3, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("pichu_tail_arc2"), -1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec2"), false, true);
    }
}

// DOWN TILT SOUND
unsafe extern "C" fn pichu_sound_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_l01"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
}

// UP TILT
unsafe extern "C" fn pichu_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.3);
        macros::ATTACK(agent, 0, 0, Hash40::new("tail1"), 5.0, 95, 85, 0, 32, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("tail2"), 5.0, 95, 85, 0, 32, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail3"), 5.0, 95, 85, 0, 32, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP TILT EFFECT
unsafe extern "C" fn pichu_effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc2"), Hash40::new("pichu_tail_arc2"), Hash40::new("top"), 1, 7, 0, 157, -80, 100, 0.85, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
    }
}

// UP TILT SOUND
unsafe extern "C" fn pichu_sound_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_tailswing"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
}

// DASH ATTACK
unsafe extern "C" fn pichu_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.7);
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 70, 100, 0, 65, 4.6, 3.0, -1.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 8.0, 70, 100, 0, 65, 3.0, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 6.0, 70, 80, 0, 60, 3.0, 3.0, -1.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 6.0, 70, 80, 0, 60, 2.5, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DASH ATTACK EFFECT
unsafe extern "C" fn pichu_effect_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec2"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec2"), false, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

// DASH ATTACK SOUND
unsafe extern "C" fn pichu_sound_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_jump01"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing04"));
    }
    wait(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing01"));
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_attack11", pichu_attack11, Low)
        .effect_acmd("effect_attack11", pichu_effect_attack11, Low)
        .sound_acmd("sound_attack11", pichu_sound_attack11, Low)

        .game_acmd("game_attacks3", pichu_attacks3, Low)
        .game_acmd("game_attacks3lw", pichu_attacks3lw, Low)

        .game_acmd("game_attacklw3", pichu_attacklw3, Low)
        .effect_acmd("effect_attacklw3", pichu_effect_attacklw3, Low)
        .sound_acmd("sound_attacklw3", pichu_sound_attacklw3, Low)

        .game_acmd("game_attackhi3", pichu_attackhi3, Low)
        .effect_acmd("effect_attackhi3", pichu_effect_attackhi3, Low)
        .sound_acmd("sound_attackhi3", pichu_sound_attackhi3, Low)

        .game_acmd("game_attackdash", pichu_attackdash, Low)
        .effect_acmd("effect_attackdash", pichu_effect_attackdash, Low)
        .sound_acmd("sound_attackdash", pichu_sound_attackdash, Low)
        
        .install();
}