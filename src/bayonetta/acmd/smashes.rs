use super::*;

//-----------------SMASH ATTACKS----------------

// UP SMASH
unsafe extern "C" fn bayonetta_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    execute(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 10, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, false, 10);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, Hash40::new("attack_hi4"), false, -1.0);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::set_power_mul_status(agent.module_accessor, 1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
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
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, Hash40::new("attack_s4_s"), false, -1.0);
        }
    }
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 10, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, false, 10);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::set_power_mul_status(agent.module_accessor, 1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) { 
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("bayonetta")
        .game_acmd("game_attackhi4", bayonetta_attackhi4, Low)

        .game_acmd("game_attacks4", bayonetta_attacks4, Low)
        .game_acmd("game_attacks4lw", bayonetta_attacks4, Low)
        .game_acmd("game_attacks4hi", bayonetta_attacks4, Low)

        .install();
}