use super::*;

//-----------------SMASH ATTACKS----------------

// UP SMASH
unsafe extern "C" fn bayonetta_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    execute(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, -1);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, false, true, 10, 3, 5, 0, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, true, 10);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, -1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::set_power_mul_status(agent.module_accessor, 1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2aa2fe5e2f), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM);
    }
}



pub fn install() {
    Agent::new("bayonetta")
        .game_acmd("game_attackhi4", bayonetta_attackhi4, Low)

        .install();
}