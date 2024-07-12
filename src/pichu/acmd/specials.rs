use super::*;

//--------------------SPECIALS-----------------------

// SIDE B - SPECIALS
unsafe extern "C" fn pichu_specials(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

// SIDE B EFFECT - SPECIALS
unsafe extern "C" fn pichu_effect_specials(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("top"), 0, 3, 9, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec"), Hash40::new("head"), 5, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 24.0);
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

// SIDE B SOUND - SPECIALS
unsafe extern "C" fn pichu_sound_specials(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_special_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_final04"));
    }
}

// SIDE B HOLD - SPECIALS_HOLD
unsafe extern "C" fn pichu_specialshold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        for _ in 0..7 {
            macros::FT_ADD_DAMAGE(agent, 0.2);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 5, 60, 0, 50, 5.5, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(-5.0), 0.2, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);        
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.6, 5, 60, 0, 30, 5.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(-5.0), 0.2, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            wait(agent.lua_state_agent, 2.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

// SIDE B HOLD EFFECT - SPECIALS_HOLD
unsafe extern "C" fn pichu_effect_specialshold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_final_sphere_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

// SIDE B HOLD SOUND - SPECIALS_HOLD
unsafe extern "C" fn pichu_sound_specialshold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_special_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_final02"));
    }
}

// SIDE B HOLD EXPRESSION - SPECIALS_HOLD
unsafe extern "C" fn pichu_expression_specialshold(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackss"), 7);
    }
}

// SIDE B END - SPECIALS_END
unsafe extern "C" fn pichu_specialsend(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 12.2, 45, 92, 0, 73, 28.0, 0.0, 8.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.5);
        AttackModule::clear_all(agent.module_accessor);
    }
}

// SIDE B END EFFECT - SPECIALS_END
unsafe extern "C" fn pichu_effect_specialsend(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("pichu_final_elec"), Hash40::new("hip"), 0, 5, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("pichu_final_explosion"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_final_elec"), false, false);
    }
}

// SIDE B END SOUND - SPECIALS_END
unsafe extern "C" fn pichu_sound_specialsend(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_final06"));
    }
}

// SIDE B END EXPRESSION - SPECIALS_END
unsafe extern "C" fn pichu_expression_specialsend(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 18, true, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosionl"), 0);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

/*
// DOWN B
unsafe extern "C" fn pichu_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
    }
}
*/

// UP B
unsafe extern "C" fn pichu_specialairhiend(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.1);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {

    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_specials", pichu_specials, Low)
        .effect_acmd("effect_specials", pichu_effect_specials, Low)
        .sound_acmd("sound_specials", pichu_sound_specials, Low)

        .game_acmd("game_specialshold", pichu_specialshold, Low)
        .effect_acmd("effect_specialshold", pichu_effect_specialshold, Low)
        .sound_acmd("sound_specialshold", pichu_sound_specialshold, Low)
        .expression_acmd("expression_specialshold", pichu_expression_specialshold, Low)

        .game_acmd("game_specialsend", pichu_specialsend, Low)
        .effect_acmd("effect_specialsend", pichu_effect_specialsend, Low)
        .sound_acmd("sound_specialsend", pichu_sound_specialsend, Low)
        .expression_acmd("expression_specialsend", pichu_expression_specialsend, Low)

        //.game_acmd("game_speciallw", pichu_speciallw, Low)

        .game_acmd("game_specialhiend", pichu_specialairhiend, Low)
        
        .install();
}