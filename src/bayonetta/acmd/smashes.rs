use super::*;

//-----------------SMASH ATTACKS----------------

// UP SMASH
unsafe extern "C" fn bayonetta_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    execute(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, false, -1);
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_HI4);
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn bayonetta_effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 20, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

// FORWARD SMASH
unsafe extern "C" fn bayonetta_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    execute(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, false, -1);
            ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_S4);
        }
    }
}

// FORWARD SMASH EFFECT
unsafe extern "C" fn bayonetta_effect_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 3, 0, 3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

// DOWN SMASH
unsafe extern "C" fn bayonetta_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    execute(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, false, -1);
            ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, *WEAPON_BAYONETTA_WICKEDWEAVELEG_STATUS_KIND_ATTACK_LW4);
        }
    }
}

// DOWN SMASH EFFECT
unsafe extern "C" fn bayonetta_effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("bayonetta")
        .game_acmd("game_attackhi4", bayonetta_attackhi4, Low)
        .effect_acmd("effect_attackhi4", bayonetta_effect_attackhi4, Low)

        .game_acmd("game_attacks4", bayonetta_attacks4, Low)
        .game_acmd("game_attacks4lw", bayonetta_attacks4, Low)
        .game_acmd("game_attacks4hi", bayonetta_attacks4, Low)
        .effect_acmd("effect_attacks4", bayonetta_effect_attacks4, Low)
        .effect_acmd("effect_attacks4lw", bayonetta_effect_attacks4, Low)
        .effect_acmd("effect_attacks4hi", bayonetta_effect_attacks4, Low)

        .effect_acmd("game_attacklw4", bayonetta_attacklw4, Low)
        .effect_acmd("effect_attacklw4", bayonetta_effect_attacklw4, Low)
    .install();
}