use super::*;

//-----------------SMASH ATTACKS----------------

// UP SMASH
unsafe extern "C" fn mario_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    execute(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 14.0, 83, 94, 0, 32, 5.0, 2.5, 1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 14.0, 83, 94, 0, 32, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}

// FORWARD SMASH CHARGE
unsafe extern "C" fn mario_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 0.5, 170, 1, 0, 20, 7.5, 10.0, 5.0, 0.0, None, None, None, 1.1, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 6.0, false);
    }
}

// FORWARD SMASH CHARGE EFFECT
unsafe extern "C" fn mario_effect_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..8 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("haver"), 1, 0, 2.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

// FORWARD SMASH
unsafe extern "C" fn mario_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 14.7, 361, 105, 0, 25, 2.0, -1.0, 0.7, 0.0, Some(-3.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 17.8, 361, 99, 0, 25, 5.0, 5.4, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if FORWARD_SMASH_CHARGE[get_entry_id(agent.module_accessor)] >= 60.0 {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 14.7, 361, 105, 0, 25, 2.0, -1.0, 0.7, 0.0, Some(-3.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 17.8, 361, 99, 0, 25, 5.0, 5.4, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

// FORWARD SMASH EFFECT
unsafe extern "C" fn mario_effect_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if FORWARD_SMASH_CHARGE[get_entry_id(agent.module_accessor)] >= 60.0 { 
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, true);
    }
}

// FORWARD SMASH SOUND
unsafe extern "C" fn mario_sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_smash_s01"));
    }
    if FORWARD_SMASH_CHARGE[get_entry_id(agent.module_accessor)] >= 60.0 { 
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
            macros::PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_mario_smash_s01"));
        }
    }
}

// FORWARD SMASH EXPRESSION
unsafe extern "C" fn mario_expression_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 6.0);
    execute(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(agent) {
            ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 16, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    if FORWARD_SMASH_CHARGE[get_entry_id(agent.module_accessor)] >= 60.0 { 
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 16, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

// DOWN SMASH
unsafe extern "C" fn mario_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 11.5, 45, 90, 0, 65, 6.0, 0.0, 0.8, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 11.5, 45, 90, 0, 65, 6.0, 0.0, 0.8, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 1.25);
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN SMASH EFFECT
unsafe extern "C" fn mario_effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.95, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 0.95, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.063, 0.215, 0.232);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, 0, 180, 0, 180, 0.95, true, *EF_FLIP_XY);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.063, 0.215, 0.232);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.95, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 0.95, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.063, 0.215, 0.232);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, 0, 180, 0, 180, 0.95, true, *EF_FLIP_XY);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.063, 0.215, 0.232);
    }
}

// DOWN SMASH SOUND
unsafe extern "C" fn mario_sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_attackair_l01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mario_attack08"));
        macros::PLAY_SE(agent, Hash40::new("se_mario_attackair_l03"));
    }
}

pub fn install() {
    Agent::new("mario")
        .game_acmd("game_attackhi4", mario_attackhi4, Low)

        .game_acmd("game_attacks4charge", mario_attacks4charge, Low)
        .effect_acmd("effect_attacks4charge", mario_effect_attacks4charge, Low)

        .game_acmd("game_attacks4", mario_attacks4, Low)
        .effect_acmd("effect_attacks4", mario_effect_attacks4, Low)
        .sound_acmd("sound_attacks4", mario_sound_attacks4, Low)
        .expression_acmd("expression_attacks4", mario_expression_attacks4, Low)

        .game_acmd("game_attacklw4", mario_attacklw4, Low)
        .effect_acmd("effect_attacklw4", mario_effect_attacklw4, Low)
        .sound_acmd("sound_attacklw4", mario_sound_attacklw4, Low)
        
        .install();
}