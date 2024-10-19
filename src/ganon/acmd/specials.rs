use super::*;

//-----------------SPECIALS-------------------

// NEUTRAL B
unsafe extern "C" fn ganon_specialn(agent: &mut L2CAgentBase) {
    let facing = PostureModule::lr(agent.module_accessor);
    if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD) {
        if facing == 1.0 {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_lw_l"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_GANON_GENERATE_ARTICLE_GSWORD, false, -1);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SWORD);
    }
}

// NEUTRAL B EFFECT
unsafe extern "C" fn ganon_effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

// NEUTRAL B SOUND
unsafe extern "C" fn ganon_sound_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_appeal_h01"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
    }
}

unsafe extern "C" fn ganon_expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {

    }
}

// SIDE B START
unsafe extern "C" fn ganon_specialsstart(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 6.8, 6.7, 7.5, 3.8);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 7.0, 7.5, 7.5);
    }
}

// SIDE B
unsafe extern "C" fn ganon_specials(agent: &mut L2CAgentBase) {
    let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
    if opponent_boma == agent.module_accessor {
        opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 361, 90, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_bury_r"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_bury_r"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 31.0); 
    if macros::is_excute(agent) {
        DamageModule::add_damage(opponent_boma, 5.0, 0);
        DamageModule::add_damage(agent.module_accessor, -5.0, 0);
    }
    frame(agent.lua_state_agent, 38.0); 
    if macros::is_excute(agent) {
        DamageModule::add_damage(opponent_boma, 5.0, 0);
        DamageModule::add_damage(agent.module_accessor, -5.0, 0);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        DamageModule::add_damage(agent.module_accessor, -5.0, 0);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_FALL);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

// SIDE B EFFECT
unsafe extern "C" fn ganon_effect_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

// SIDE B SOUND
unsafe extern "C" fn ganon_sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ganon_special_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s03"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

// SIDE B EXPRESSION
unsafe extern "C" fn ganon_expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// AERIAL SIDE B
unsafe extern "C" fn ganon_specialairs(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.0, 361, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::SLOW_OPPONENT(agent, 3.0, 40.0);
    }
}

// DOWN B
unsafe extern "C" fn ganon_speciallw(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("kneer"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 6.0, 8.5, 9.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 2.0, 6.0, 8.5, 10.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 25, 65, 0, 65, 3.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0, 25, 65, 0, 65, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
}

// DOWN B CONTINUE
unsafe extern "C" fn ganon_speciallwhold(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("kneer"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    macros::SET_SPEED_EX(agent, 2.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 25, 65, 0, 65, 3.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0, 25, 65, 0, 65, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 2.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
}

// DOWN B CONTINUE EFFECT
unsafe extern "C" fn ganon_effect_speciallwhold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneer"), 12, -1.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        macros::FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

// DOWN B CONTINUE EXPRESSION
unsafe extern "C" fn ganon_expression_speciallwhold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// UP B 2 START
unsafe extern "C" fn ganon_sound_specialhi2_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_assist_vanish"));
    }
}

// UP B 2 START EFFECT
unsafe extern "C" fn ganon_effect_specialhi2_start(agent: &mut L2CAgentBase) { 
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

// UP B 2
unsafe extern "C" fn ganon_specialhi2(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) { 
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

// UP B 2 EFFECT
unsafe extern "C" fn ganon_effect_specialhi2(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);

    }
}

// UP B 2 SOUND
unsafe extern "C" fn ganon_sound_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialn", ganon_specialn, Low)
        .effect_acmd("effect_specialn", ganon_effect_specialn, Low)
        .sound_acmd("sound_specialn", ganon_sound_specialn, Low)
        .expression_acmd("expression_specialn", ganon_expression_specialn, Low)

        .game_acmd("game_specialsstart", ganon_specialsstart, Low)

        .game_acmd("game_specials", ganon_specials, Low)
        .effect_acmd("effect_specials", ganon_effect_specials, Low)
        .sound_acmd("sound_specials", ganon_sound_specials, Low)
        .expression_acmd("expression_specials", ganon_expression_specials, Low)

        .game_acmd("game_specialairs", ganon_specialairs, Low)

        .game_acmd("game_speciallw", ganon_speciallw, Low)

        .game_acmd("game_speciallwhold", ganon_speciallwhold, Low)
        .effect_acmd("effect_speciallwhold", ganon_effect_speciallwhold, Low)
        .expression_acmd("expression_speciallwhold", ganon_expression_speciallwhold, Low)

        .effect_acmd("effect_specialhi2_start", ganon_effect_specialhi2_start, Low)
        .sound_acmd("sound_specialhi2_start", ganon_sound_specialhi2_start, Low)

        .game_acmd("game_specialhi2", ganon_specialhi2, Low)
        .effect_acmd("effect_specialhi2", ganon_effect_specialhi2, Low)
        .sound_acmd("sound_specialhi2", ganon_sound_specialhi2, Low)

        .install();
}