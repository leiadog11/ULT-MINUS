use super::*;

//-------------------SPECIALS------------------

// NEUTRAL SPECIAL
unsafe extern "C" fn captain_specialn(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if GUN_COOLDOWN[ENTRY_ID] <= 0 {
            GUN_COOLDOWN[ENTRY_ID] = 480;
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_RAYGUN), 0, 0, false, false);
        }
    }
}

// NEUTRAL SPECIAL EFFECT
unsafe extern "C" fn captain_effect_specialn(agent: &mut L2CAgentBase) {
}

// NEUTRAL SPECIAL SOUND
unsafe extern "C" fn captain_sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
}

// NEUTRAL SPECIAL EXPRESSION
unsafe extern "C" fn captain_expression_specialn(agent: &mut L2CAgentBase) {
}

// SIDE SPECIAL
unsafe extern "C" fn captain_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.1);
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 12.0);
    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 9.0, 8.8, Some(0.0), Some(5.0), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 12.0, 8.8, Some(0.0), Some(5.0), Some(8.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

// AERIAL SIDE SPECIAL
unsafe extern "C" fn captain_specialairsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.1);
        JostleModule::set_status(agent.module_accessor, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 19.0);
    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.1, 0.0, 12.0, 11.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    macros::FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP SPECIAL
unsafe extern "C" fn captain_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 11.0, 361, 75, 0, 65, 11.5, 10.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 19.0);
}

// UP SPECIAL EFFECT
unsafe extern "C" fn captain_effect_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 7.5, 12, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

// UP SPECIAL SOUND
unsafe extern "C" fn captain_sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_captain_special_h03"));
        macros::PLAY_SE(agent, Hash40::new("vc_captain_appeal03"));
    }
    wait(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
}

// UP SPECIAL EXPRESSION
unsafe extern "C" fn captain_expression_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// DOWN SPECIAL
unsafe extern "C" fn captain_speciallw(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 1.0);
    KICK_SPEED[ENTRY_ID] = smash::app::sv_math::rand(hash40("agent"), 4) as u64;
    macros::FT_MOTION_RATE(agent, 0.857);
    frame(agent.lua_state_agent, 15.0);
    if KICK_SPEED[ENTRY_ID] == 0 {
        macros::FT_MOTION_RATE(agent, 1.75);
    }
    else if KICK_SPEED[ENTRY_ID] == 1 {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
    else if KICK_SPEED[ENTRY_ID] == 2 {
        macros::FT_MOTION_RATE(agent, 0.55);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 15.0, 52, 88, 0, 60, 3.8, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        JostleModule::set_status(agent.module_accessor, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 11.0, 52, 60, 0, 90, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 7.0, 46, 65, 0, 80, 3.2, 10.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

// DOWN SPECIAL SOUND
unsafe extern "C" fn captain_sound_speciallw(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        SoundModule::play_se(agent.module_accessor, Hash40::new("vc_captain_003"), true, false, false, false, enSEType(0));
        if KICK_SPEED[ENTRY_ID] == 0 { 
            SoundModule::set_se_pitch_ratio(agent.module_accessor, Hash40::new("vc_captain_003"), 0.65);
        }
        else if KICK_SPEED[ENTRY_ID] == 2 { 
            SoundModule::set_se_pitch_ratio(agent.module_accessor, Hash40::new("vc_captain_003"), 1.35);
        }
    }
    wait(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_captain_special_l01"));
    }
}

// AERIAL DOWN SPECIAL
unsafe extern "C" fn captain_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.867);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 15.0, 361, 57, 0, 80, 5.76, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 361, 46, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 361, 46, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN SPECIAL 2
unsafe extern "C" fn captain_speciallw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        
    }
}

// DOWN SPECIAL 2 EFFECT
unsafe extern "C" fn captain_effect_speciallw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

// DOWN SPECIAL 2 SOUND
unsafe extern "C" fn captain_sound_speciallw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_captain_004"));
        macros::PLAY_SE(agent, Hash40::new("se_captain_escape"));
    }
}

// DOWN SPECIAL 2 EXPRESSION
unsafe extern "C" fn captain_expression_speciallw2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


pub fn install() {
    Agent::new("captain")
        .game_acmd("game_specialn", captain_specialn, Low)
        .effect_acmd("effect_specialn", captain_effect_specialn, Low)
        .sound_acmd("sound_specialn", captain_sound_specialn, Low)
        .expression_acmd("expression_specialn", captain_expression_specialn, Low)

        .game_acmd("game_specialsstart", captain_specialsstart, Low)

        .game_acmd("game_specialairsstart", captain_specialairsstart, Low)

        .game_acmd("game_specialhi", captain_specialhi, Low)
        .effect_acmd("effect_specialhi", captain_effect_specialhi, Low)
        .sound_acmd("sound_specialhi", captain_sound_specialhi, Low)
        .expression_acmd("expression_specialhi", captain_expression_specialhi, Low)

        .game_acmd("game_speciallw", captain_speciallw, Low)
        .sound_acmd("sound_speciallw", captain_sound_speciallw, Low)
        .game_acmd("game_specialairlw", captain_specialairlw, Low)

        .game_acmd("game_speciallw2", captain_speciallw2, Low)
        .effect_acmd("effect_speciallw2", captain_effect_speciallw2, Low)
        .sound_acmd("sound_speciallw2", captain_sound_speciallw2, Low)
        .expression_acmd("expression_speciallw2", captain_expression_speciallw2, Low)

        .install();
}