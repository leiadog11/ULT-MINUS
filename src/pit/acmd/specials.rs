use super::*;

static mut OPP_X: [f32; 8] = [0.0; 8];
static mut OPP_Y: [f32; 8] = [0.0; 8];
static mut OPP_Z: [f32; 8] = [0.0; 8];

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
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        let opponent_bomas = get_opponent_bomas(agent.module_accessor);
        OPP_X[ENTRY_ID] = PostureModule::pos_x(opponent_bomas[0]);
        OPP_Y[ENTRY_ID] = PostureModule::pos_y(opponent_bomas[0]);
        OPP_Z[ENTRY_ID] = PostureModule::pos_z(opponent_bomas[0]);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 88, 84, 0, 53, 10.0, OPP_Z[ENTRY_ID], OPP_Y[ENTRY_ID], OPP_X[ENTRY_ID], None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::EFFECT_FOLLOW(agent, Hash40::new("pit_pa_fly_arrow"), Hash40::new("top"), OPP_Z[ENTRY_ID], OPP_Y[ENTRY_ID], OPP_X[ENTRY_ID], 90, 0, 0, 100, true);
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

// DOWN B
unsafe extern "C" fn pit_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_CHARIOTSIGHT, false, -1);
    }
}

// UP B
unsafe extern "C" fn pit_specialhi(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) { 
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 75, 100, 50, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
}

// UP B FLIGHT
unsafe extern "C" fn pit_effect_specialhiflight(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_fly_miracle_b"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}   

pub fn install() {
    Agent::new("pit")
        .game_acmd("game_specialnstart", pit_specialnstart, Low)

        .game_acmd("game_specialnfires", pit_specialnfires, Low)

        .game_acmd("game_specialnfirehi", pit_specialnfirehi, Low)
        .effect_acmd("effect_specialnfirehi", pit_effect_specialnfirehi, Low)
        .sound_acmd("sound_specialnfirehi", pit_sound_specialnfirehi, Low)

        .game_acmd("game_speciallwstartr", pit_speciallwstart, Low)
        .game_acmd("game_speciallwstartl", pit_speciallwstart, Low)

        .game_acmd("game_specialhi", pit_specialhi, Low)

        .effect_acmd("effect_specialhiflight", pit_effect_specialhiflight, Low)

        .install();
}