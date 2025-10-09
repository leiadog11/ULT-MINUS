use super::*;

//---------------AERIALS----------------

// NAIR - NEUTRAL AIR
unsafe extern "C" fn pichu_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 1.1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 7.0, 361, 72, 0, 15, 4.0, 5.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        macros::HIT_NODE(agent, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("mimil1"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 5.0, 361, 112, 0, 0, 4.0, 5.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// NAIR EFFECT
unsafe extern "C" fn pichu_effect_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 45, -90, 0.8, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, false);
    }
}

// NAIR SOUND
unsafe extern "C" fn pichu_sound_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        macros::PLAY_STATUS(agent, Hash40::new("se_pichu_attackair_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
}

// FAIR - FORWARD AIR
unsafe extern "C" fn pichu_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_ADD_DAMAGE(agent, 1.6);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 3.5, 366, 100, 20, 0, 6.8, 2.2, 0.5, 0.0, None, None, None, 0.5, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 3.5, 366, 100, 20, 0, 5.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            CancelModule::enable_cancel(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 3.5, 45, 140, 0, 55, 6.8, 2.2, 0.5, 0.0, None, None, None, 1.0, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 3.5, 45, 140, 0, 55, 5.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// BAIR - BACK AIR
unsafe extern "C" fn pichu_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 2.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 4.0, 0.0, 1.8, -11.5, Some(0.0), Some(1.8), Some(0.5), 0.5, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 3.5, 0.0, 2.5, -10.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 3.5, 0.0, 2.5, -10.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.5, 42, 160, 0, 70, 5.0, 0.0, 3.0, -10.5, Some(0.0), Some(3.0), Some(-0.5), 1.0, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// UP AIR
unsafe extern "C" fn pichu_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 1.4);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("tail1"), 4.0, 80, 60, 0, 60, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("tail3"), 4.0, 80, 60, 0, 60, 3.6, 1.5, -0.2, -0.5, None, None, None, 1.0, 3.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// UP AIR EFFECT
unsafe extern "C" fn pichu_effect_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc"), Hash40::new("pichu_tail_arc"), Hash40::new("top"), 0, 6, 2, 0, -130, -90, 0.85, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("tail3"), 2, 0, 3, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec"), false, true);
    }
}

// UP AIR SOUND
unsafe extern "C" fn pichu_sound_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_tailswing"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
    }
}

// DOWN AIR - DAIR
unsafe extern "C" fn pichu_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 1.7);
        macros::HIT_NODE(agent, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("mimil1"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 9.0, 270, 86, 0, 16, 5.5, 4.5, -1.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 12.0, 361, 100, 0, 20, 5.4, 4.5, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 12.0, 361, 100, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// BACK AIR / FORWARD AIR / UP AIR / NEUTRAL AIR LANDING
unsafe extern "C" fn pichu_landingairbfhin(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

// DOWN AIR LANDING
unsafe extern "C" fn pichu_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.5);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 4.0, 0.0, 4.0, -6.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_attackairn", pichu_attackairn, Low)
        .effect_acmd("effect_attackairn", pichu_effect_attackairn, Low)
        .sound_acmd("sound_attackairn", pichu_sound_attackairn, Low)

        .game_acmd("game_attackairf", pichu_attackairf, Low)

        .game_acmd("game_attackairb", pichu_attackairb, Low)

        .game_acmd("game_attackairhi", pichu_attackairhi, Low)
        .effect_acmd("effect_attackairhi", pichu_effect_attackairhi, Low)
        .sound_acmd("sound_attackairhi", pichu_sound_attackairhi, Low)

        .game_acmd("game_attackairlw", pichu_attackairlw, Low)

        .game_acmd("game_landingairb", pichu_landingairbfhin, Low)
        .game_acmd("game_landingairf", pichu_landingairbfhin, Low)
        .game_acmd("game_landingairhi", pichu_landingairbfhin, Low)
        .game_acmd("game_landingairn", pichu_landingairbfhin, Low)

        .game_acmd("game_landingairlw", pichu_landingairlw, Low)
        
        .install();
}