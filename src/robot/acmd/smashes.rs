use super::*;

//-----------------SMASH ATTACKS------------------

// FORWARD SMASH
unsafe extern "C" fn rob_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.05);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        for _ in 0..30 {
            macros::SET_SPEED_EX(agent, 3.5, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 50, 0, 30, 3.3, 0.0, 13.0, 12.0, Some(0.0), Some(6.0), Some(12.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 367, 50, 0, 30, 3.3, 0.0, 13.0, -8.0, Some(0.0), Some(6.0), Some(-8.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);            wait(agent.lua_state_agent, 4.0);
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                AttackModule::clear_all(agent.module_accessor);
            }
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.5, 41, 105, 0, 60, 10.3, 0.0, 10.3, 14.0, Some(0.0), Some(10.3), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_ROBOT_REFLECTOR_KIND_ARMSPIN, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

// FORWARD SMASH EFFECT
unsafe extern "C" fn rob_effect_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_armspin"), Hash40::new("body"), 1, 0, 0, 0, 60, 90, 1, true);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 0 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.196, 0.196, 0.216);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.22, 0.059, 0.039);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 2 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.176, 0.137, 0.059);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.235, 0.196, 0.255);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.098, 0.157, 0.196);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.098, 0.059, 0);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.098, 0.098, 0.157);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.118, 0.039, 0.051);
        }
    }
}

// FORWARD SMASH SOUND
unsafe extern "C" fn rob_sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_special_s02"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_special_s03"));
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_special_s03"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_special_s03"));
    }
    wait(agent.lua_state_agent, 5.0);
}

// FORWARD SMASH EXPRESSION
unsafe extern "C" fn rob_expression_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 4);
    }
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 7.0);
    for _ in 0..30 {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 6);
            wait(agent.lua_state_agent, 1.0);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

// UP SMASH
unsafe extern "C" fn rob_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 115, 100, 150, 0, 8, 0.0, 4.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 115, 100, 150, 0, 8, 0.0, 4.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 80, 93, 0, 40, 8.7, 0.0, 25.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 85, 93, 0, 40, 6.2, 0.0, 34.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN SMASH
unsafe extern "C" fn rob_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(agent, 1.2);
    for _ in 0..2 {
        frame(agent.lua_state_agent, 7.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 90, 20, 0, 30, 4.0, 0.0, 5.5, -3.0, Some(0.0), Some(5.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 90, 20, 0, 30, 4.0, 0.0, 5.5, -13.0, Some(0.0), Some(5.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        wait(agent.lua_state_agent, 1.0);
        AttackModule::clear_all(agent.module_accessor);
    }
    for _ in 0..2 {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 90, 20, 0, 30, 4.0, 0.0, 7.5, -3.0, Some(0.0), Some(7.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 90, 20, 0, 30, 4.0, 0.0, 7.5, -13.0, Some(0.0), Some(7.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        wait(agent.lua_state_agent, 1.0);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("body"), 11.0, 72, 95, 0, 50, 8.5, -1.5, 13.0, 13.0, Some(-1.5), Some(0.0), Some(-13.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 170, 110, 0, 85, 3.0, 0.0, 1.5, -3.0, Some(0.0), Some(1.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 170, 110, 0, 85, 3.0, 0.0, 1.5, -13.0, Some(0.0), Some(1.5), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN SMASH EFFECT
unsafe extern "C" fn rob_effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 7, 0, 0, -40, 0, 1.4, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 3);
        macros::EFFECT(agent, Hash40::new("robot_roboburner_start"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("robot_roboburner_ring"), Hash40::new("top"), 0, 0, 0, 0, 90, -90, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_roboburner"), Hash40::new("knee"), 0, 0, 0, 0, 90, -90, 1, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

// DOWN SMASH SOUND
unsafe extern "C" fn rob_sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_robot_attack07"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_07"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_swing_m"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_swing_m"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_swing_m"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_swing_m"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_robot_machine_up"));
    }
}

// DOWN SMASH EXPRESSION
unsafe extern "C" fn rob_expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 2.0);
    execute(agent.lua_state_agent, 2.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_spinattacks"), 22, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 6);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 7);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TPART, 3);
    }
}

pub fn install() {
    Agent::new("robot")
        .game_acmd("game_attacks4", rob_attacks4, Low)
        .game_acmd("game_attacks4lw", rob_attacks4, Low)
        .game_acmd("game_attacks4hi", rob_attacks4, Low)
        .effect_acmd("effect_attacks4", rob_effect_attacks4, Low)
        .effect_acmd("effect_attacks4lw", rob_effect_attacks4, Low)
        .effect_acmd("effect_attacks4hi", rob_effect_attacks4, Low)
        .sound_acmd("sound_attacks4", rob_sound_attacks4, Low)
        .sound_acmd("sound_attacks4lw", rob_sound_attacks4, Low)
        .sound_acmd("sound_attacks4hi", rob_sound_attacks4, Low)
        .expression_acmd("expression_attacks4", rob_expression_attacks4, Low)
        .expression_acmd("expression_attacks4lw", rob_expression_attacks4, Low)
        .expression_acmd("expression_attacks4hi", rob_expression_attacks4, Low)

        .game_acmd("game_attacklw4", rob_attacklw4, Low)
        .effect_acmd("effect_attacklw4", rob_effect_attacklw4, Low)
        .sound_acmd("sound_attacklw4", rob_sound_attacklw4, Low)
        .expression_acmd("expression_attacklw4", rob_expression_attacklw4, Low)

        .game_acmd("game_attackhi4", rob_attackhi4, Low)
        .install();
}