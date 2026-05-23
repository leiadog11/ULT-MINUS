use super::*;

//-------------------SPECIALS------------------

// SIDE B
unsafe extern "C" fn roy_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_ROY_GENERATE_ARTICLE_ROYSWORD, false, -1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 55, 100, 10, 60, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(12.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 55, 100, 10, 50, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(12.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 25.0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

// SIDE B EFFECT
unsafe extern "C" fn roy_effect_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_blazeend_throw"), Hash40::new("top"), 0, 12, 4, 50, -93, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
       // macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 9, 14, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_catch_hand"), false, true);
    }
}

// SIDE B SOUND
unsafe extern "C" fn roy_sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_eflame_special_s01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("vc_eflame_special_s01_rand"));
    }
}

// SIDE B EXPRESSION
unsafe extern "C" fn roy_expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// DOWN B ROLL
unsafe extern "C" fn roy_speciallwroll(agent: &mut L2CAgentBase) { 
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        JostleModule::set_status(agent.module_accessor, false);
    } 
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 90, 72, 0, 54, 8.2, 0.0, 7.8, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 90, 42, 0, 38, 7.8, 0.0, 7.8, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

// DOWN B ROLL EFFECT
unsafe extern "C" fn roy_effect_speciallwroll(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.85);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 6, 0, 0, 0, -90, 0.9, false, 0.2);
        macros::LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 5, 0, 0, 0, -90, 0.9, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.6);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
}

// DOWN B ROLL SOUND
unsafe extern "C" fn roy_sound_speciallwroll(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 10.0);
    let rand = smash::app::sv_math::rand(hash40("agent"), 10) as u64;
    if rand < 6 {
        if macros::is_excute(agent) { 
            macros::PLAY_SE(agent, Hash40::new("vc_roy_attack01"));
        }
    }
    if macros::is_excute(agent) { 
        macros::PLAY_SE(agent, Hash40::new("se_roy_escape"));
    }
}

// DOWN B ROLL EXPRESSION
unsafe extern "C" fn roy_expression_speciallwroll(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

// DOWN B DIVE
unsafe extern "C" fn roy_speciallwdive(agent: &mut L2CAgentBase) { 
    macros::FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 11.0, 240, 100, 0, 70, 6.8, 0.0, -0.7, 0.4, Some(0.0), Some(-2.5), Some(0.4), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN B DIVE EFFECT
unsafe extern "C" fn roy_effect_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) { 
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 7, Hash40::new("sword1"), 0, 0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 1.05, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) { 
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_sword_light"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) { 
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

// DOWN B DIVE SOUND
unsafe extern "C" fn roy_sound_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        macros::PLAY_SE(agent, Hash40::new("se_roy_attackdash"));
    }
    frame(agent.lua_state_agent, 12.0);
    let rand = smash::app::sv_math::rand(hash40("agent"), 10) as u64;
    if rand < 6 {
        if macros::is_excute(agent) { 
            macros::PLAY_SE(agent, Hash40::new("vc_roy_attack04"));
        }
    }
    if macros::is_excute(agent) { 
        macros::PLAY_SE(agent, Hash40::new("se_roy_attackair_l01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_fire_s"));
    }
}

// DOWN B DIVE EXPRESSION
unsafe extern "C" fn roy_expression_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
 
    }
}

// DOWN B LANDING
unsafe extern "C" fn roy_speciallwlanding(agent: &mut L2CAgentBase) { 
    macros::FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 1.0);
}

// DOWN B LANDING EFFECT
unsafe extern "C" fn roy_effect_speciallwlanding(agent: &mut L2CAgentBase) { 
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

// DOWN B LANDING SOUND
unsafe extern "C" fn roy_sound_speciallwlanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_roy_landing02"));
    }
}

// DOWN B LANDING EXPRESSIONG
unsafe extern "C" fn roy_expression_speciallwlanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

// UP B
unsafe extern "C" fn roy_specialhi(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 84, 100, 120, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.0, 3.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 6.0, 0.0, 17.0, 10.0, None, None, None, 1.0, 3.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 5.3, 0.0, 12.0, 10.0, None, None, None, 1.0, 3.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if !UP_B_USED[ENTRY_ID] {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 8.0, 70, 93, 0, 75, 6.5, 0.0, 17.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 2.3, 3.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

pub fn install() {
    Agent::new("roy")
        .game_acmd("game_specials", roy_specials, Low)
        .effect_acmd("effect_specials", roy_effect_specials, Low)
        .sound_acmd("sound_specials", roy_sound_specials, Low)
        .expression_acmd("expression_specials", roy_expression_specials, Low)

        .game_acmd("game_speciallwroll", roy_speciallwroll, Low)
        .effect_acmd("effect_speciallwroll", roy_effect_speciallwroll, Low)
        .sound_acmd("sound_speciallwroll", roy_sound_speciallwroll, Low)
        .expression_acmd("expression_speciallwroll", roy_expression_speciallwroll, Low)

        .game_acmd("game_speciallwdive", roy_speciallwdive, Low)
        .effect_acmd("effect_speciallwdive", roy_effect_speciallwdive, Low)
        .sound_acmd("sound_speciallwdive", roy_sound_speciallwdive, Low)
        .expression_acmd("expression_speciallwdive", roy_expression_speciallwdive, Low)

        .game_acmd("game_speciallwlanding", roy_speciallwlanding, Low)
        .effect_acmd("effect_speciallwlanding", roy_effect_speciallwlanding, Low)
        .sound_acmd("sound_speciallwlanding", roy_sound_speciallwlanding, Low)
        .expression_acmd("expression_speciallwlanding", roy_expression_speciallwlanding, Low)

        .game_acmd("game_specialhi", roy_specialhi, Low)
        .game_acmd("game_specialairhi", roy_specialhi, Low)

        .install();
}