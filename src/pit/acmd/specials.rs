use super::*;

//-------------------SPECIALS------------------

// NEUTRAL B START
unsafe extern "C" fn pit_specialnstart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, false, -1);
    }
}

// NEUTRAL B FIRE S
unsafe extern "C" fn pit_specialnfires(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

// NEUTRAL B FIRE HI
unsafe extern "C" fn pit_specialnfirehi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            let opponent_bomas = get_opponent_bomas(agent.module_accessor);
            for opponent_boma in opponent_bomas.iter() {
                let the_one_x = PostureModule::pos_x(*opponent_boma);
                let the_one_y = PostureModule::pos_y(*opponent_boma);
                let the_one_z = PostureModule::pos_z(*opponent_boma);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 88, 84, 0, 53, 6.0, the_one_x - 20.0, the_one_y - 100.0, the_one_z - 20.0, Some(the_one_x + 20.0), Some(the_one_y + 100.0), Some(the_one_z + 20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::EFFECT_FOLLOW(agent, Hash40::new("pit_pa_fly_arrow"), Hash40::new("top"), the_one_x, the_one_y, the_one_z, 90, 0, 0, 100, true);
            } 
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) { 
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

// NEUTRAL B FIRE HI EFFECT
unsafe extern "C" fn pit_effect_specialnfirehi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_pa_hold_bow"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_pa_hold_bow2"), true, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("pit_pa_max_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(agent, Hash40::new("pit_pa_max"), Hash40::new("top"), 0, 34, -1, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) { 
            macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_pa_fly_arrow"), true, true);
        }
    }
}

// NEUTRAL B FIRE HI SOUND
unsafe extern "C" fn pit_sound_specialnfirehi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_pit_special_n01"));
        macros::PLAY_SE(agent, Hash40::new("vc_pit_special_n01"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pit_special_n02"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_pit_special_n02"), 30);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pit_special_n03"));
        }
    }
}

// SPECIAL LW SHIELD INSTALL EFFECT
unsafe extern "C" fn pit_effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.5, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
}

// SPECIAL LW SHIELD INSTALL SOUND
unsafe extern "C" fn pit_sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        // YELL SHIELD
        macros::PLAY_SE(agent, Hash40::new("se_pit_special_l01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pit_rnd_special_l"));
    }
}

// SPECIAL LW SHIELD INSTALL EXPRESSION
unsafe extern "C" fn pit_expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// --EDIT MOTION LIST FOR THE MOTION BELOW AND START--

// SPECIAL LW END EFFECT
unsafe extern "C" fn pit_effect_speciallwend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("pit_guardian_shield_end"), Hash40::new("virtualguardianf"), -2, 3, 4, 0, 240, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("pit_guardian_shield_end"), Hash40::new("virtualguardianb"), -2, 3, -4, 0, 120, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_guardian_shield"), false, false);
    }
}

// SPECIAL LW END EXPRESSION
unsafe extern "C" fn pit_expression_speciallwend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamss"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("pit")
        .game_acmd("game_specialnstart", pit_specialnstart, Low)

        .game_acmd("game_specialnfires", pit_specialnfires, Low)

        .game_acmd("game_specialnfirehi", pit_specialnfirehi, Low)
        .effect_acmd("effect_specialnfirehi", pit_effect_specialnfirehi, Low)
        .sound_acmd("sound_specialnfirehi", pit_sound_specialnfirehi, Low)

        .effect_acmd("effect_speciallw", pit_effect_speciallw, Low)
        .sound_acmd("sound_speciallw", pit_sound_speciallw, Low)
        .expression_acmd("expression_speciallw", pit_expression_speciallw, Low)

        .effect_acmd("effect_speciallwend", pit_effect_speciallwend, Low)
        .expression_acmd("expression_speciallwend", pit_expression_speciallwend, Low)

        .install();
}