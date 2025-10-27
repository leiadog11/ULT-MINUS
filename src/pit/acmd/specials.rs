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
    let mut dist_x = 0.0;
    let mut dist_y = 0.0;
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        let opponent_bomas = get_opponent_bomas(agent.module_accessor);
        OPP_X[ENTRY_ID] = PostureModule::pos_x(opponent_bomas[0]);
        OPP_Y[ENTRY_ID] = PostureModule::pos_y(opponent_bomas[0]);

        let PLAYER_X = PostureModule::pos_x(agent.module_accessor);
        let PLAYER_Y = PostureModule::pos_y(agent.module_accessor);

        dist_x = OPP_X[ENTRY_ID] - PLAYER_X;
        dist_y = OPP_Y[ENTRY_ID] - PLAYER_Y;
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        let lr = PostureModule::lr(agent.module_accessor);
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 88, 84, 0, 53, 10.0, 0.0, dist_y + 4.0, dist_x * lr, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::EFFECT_FOLLOW(agent, Hash40::new("pit_pa_fly_arrow"), Hash40::new("top"), 10, dist_y, dist_x * lr, 90, 0, 0, 60, true);
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
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    if !SHIELD_ON[ENTRY_ID] {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_CHARIOT, false, -1);
        }
    }
}

// DOWN B EFFECT
unsafe extern "C" fn pit_effect_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {

    }
}

// DOWN B SOUND
unsafe extern "C" fn pit_sound_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {

    }
}

// DOWN B EXPRESSION
unsafe extern "C" fn pit_expression_speciallwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {

    }
}

// UP B START
unsafe extern "C" fn pit_specialhistart(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) { 
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 70, 80, 70, 0, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 2.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
    }
}

// UP B START EFFECT
unsafe extern "C" fn pit_effect_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_fly_miracle_start"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

// UP B START SOUND
unsafe extern "C" fn pit_sound_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let se = SoundModule::play_se(agent.module_accessor, Hash40::new("se_pit_flight_start"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, se as i32, 4.75, 0);
    }
}

// UP B FLIGHT EFFECT
unsafe extern "C" fn pit_effect_specialhiflight(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..100 { 
        if macros::is_excute(agent) { 
            macros::EFFECT_FOLLOW(agent, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingl1"), -3, 0, 1, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("pit_ikaros_wing_flare"), Hash40::new("s_wingr1"), -3, 0, -1, 0, 0, 0, 1, false);
        }
        wait(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) { 
            macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_ikaros_wing_flare"), true, true);
        }
    }
} 

// UP B FLIGHT SOUND
unsafe extern "C" fn pit_sound_specialhiflight(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..100 { 
        if macros::is_excute(agent) { 
            if !SoundModule::is_playing(agent.module_accessor, Hash40::new("se_pit_flight_wings")) { 
                let se = SoundModule::play_se(agent.module_accessor, Hash40::new("se_pit_flight_wings"), true, false, false, false, enSEType(0));
                SoundModule::set_se_vol(agent.module_accessor, se as i32, 6.5, 0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
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
        .effect_acmd("effect_speciallwstartr", pit_effect_speciallwstart, Low)
        .effect_acmd("effect_speciallwstartl", pit_effect_speciallwstart, Low)
        .sound_acmd("sound_speciallwstartr", pit_sound_speciallwstart, Low)
        .sound_acmd("sound_speciallwstartl", pit_sound_speciallwstart, Low)
        .expression_acmd("expression_speciallwstartr", pit_expression_speciallwstart, Low)
        .expression_acmd("expression_speciallwstartl", pit_expression_speciallwstart, Low)

        .game_acmd("game_specialhistart", pit_specialhistart, Low)
        .effect_acmd("effect_specialhistart", pit_effect_specialhistart, Low)
        .sound_acmd("sound_specialhistart", pit_sound_specialhistart, Low)

        .effect_acmd("effect_specialhiflight", pit_effect_specialhiflight, Low)
        .sound_acmd("sound_specialhiflight", pit_sound_specialhiflight, Low)

        .install();
}