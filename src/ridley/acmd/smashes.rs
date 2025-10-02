use super::*;

//-----------------SMASH ATTACKS----------------

// FORWARD SMASH
unsafe extern "C" fn ridley_attacks4(agent: &mut L2CAgentBase) {
    if AURA[get_entry_id(agent.module_accessor)] { 
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
    }
    let mut fire_z = 14.0;
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 7.0);
    }
    if AURA[get_entry_id(agent.module_accessor)] { 
        frame(agent.lua_state_agent, 18.0);
        for _ in 0..3 { 
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 14.0, 0.0, 8.0, fire_z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            }
            wait(agent.lua_state_agent, 6.0);
            if macros::is_excute(agent) {
                fire_z = fire_z + 8.0;
                AttackModule::clear_all(agent.module_accessor);
            }
        }
    }
    else {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 14.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 5.0);
    }
}

// FORWARD SMASH EFFECT
unsafe extern "C" fn ridley_effect_attacks4(agent: &mut L2CAgentBase) {
    let mut fire_z = 15;
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    if AURA[get_entry_id(agent.module_accessor)] { 
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) { 
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        for _ in 0..3 { 
            if macros::is_excute(agent) { 
                macros::EFFECT(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, fire_z, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
            }
            wait(agent.lua_state_agent, 6.0);
            fire_z = fire_z + 8;
        }
    }
    else {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

// FORWARD SMASH SOUND
unsafe extern "C" fn ridley_sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_smash_s01"));
        macros::PLAY_SE(agent, Hash40::new("vc_ridley_special_s02"));
    }
    if AURA[get_entry_id(agent.module_accessor)] { 
        frame(agent.lua_state_agent, 18.0);
        for _ in 0..3 { 
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
            }
            wait(agent.lua_state_agent, 6.0);
        }
    }
    else {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
        }
    }
}

// FORWARD SMASH EXPRESSION
unsafe extern "C" fn ridley_expression_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    execute(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    if AURA[get_entry_id(agent.module_accessor)] { 
        frame(agent.lua_state_agent, 18.0);
        for _ in 0..3 { 
            if macros::is_excute(agent) { 
                macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
                ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
                macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            }
            wait(agent.lua_state_agent, 6.0);
        }
    }
    else {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
    }

}

// DOWN SMASH
unsafe extern "C" fn ridley_attacklw4(agent: &mut L2CAgentBase) {
    if AURA[get_entry_id(agent.module_accessor)] { 
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 270, 66, 0, 40, 8.0, 0.0, 5.0, 21.0, Some(0.0), Some(5.0), Some(-21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN SMASH EXPRESSION
unsafe extern "C" fn ridley_expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    execute(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {       
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

// UP SMASH
unsafe extern "C" fn ridley_attackhi4(agent: &mut L2CAgentBase) {
    if AURA[get_entry_id(agent.module_accessor)] { 
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 17.0, 82, 78, 0, 58, 8.5, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 17.0, 82, 78, 0, 58, 8.5, 0.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 17.0, 82, 78, 0, 58, 9.0, -1.5, -1.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 17.0, 315, 78, 0, 58, 8.5, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 17.0, 315, 78, 0, 58, 8.5, 0.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 17.0, 315, 78, 0, 58, 9.0, -1.5, -1.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_attacks4", ridley_attacks4, Low)
        .effect_acmd("effect_attacks4", ridley_effect_attacks4, Low)
        .sound_acmd("sound_attacks4", ridley_sound_attacks4, Low)
        .expression_acmd("expression_attacks4", ridley_expression_attacks4, Low)

        .game_acmd("game_attacklw4", ridley_attacklw4, Low)
        .expression_acmd("expression_attacklw4", ridley_expression_attacklw4, Low)

        .game_acmd("game_attackhi4", ridley_attackhi4, Low)

        .install();
}