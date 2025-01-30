use super::*;

//--------------ITEM THROW----------------

//ITEM THROW 
unsafe extern "C" fn rob_effect_itemlightthrow(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GYRO_LIFE[get_entry_id(agent.module_accessor)] = 360;
    }
}

//-----------------TAUNTS--------------------

// UP TAUNT
unsafe extern "C" fn rob_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent){
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_BURY, true);
    }
}

/*
// SIDE TAUNT
unsafe extern "C" fn rob_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent){
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROBOT_GENERATE_ARTICLE_BEAM, false, -1);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent){
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROBOT_GENERATE_ARTICLE_BEAM, false, -1);
    }
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent){
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROBOT_GENERATE_ARTICLE_BEAM, false, -1);
    }
}
*/

pub fn install() {
    Agent::new("robot")
        .effect_acmd("effect_itemlightthrowf", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowf4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairf", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairf4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowb", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowb4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairb", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairb4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowlw", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowlw4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairlw", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairlw4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowhi", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowhi4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairhi", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowairhi4", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowdash", rob_effect_itemlightthrow, Low)
        .effect_acmd("effect_itemlightthrowdrop", rob_effect_itemlightthrow, Low)

        .game_acmd("game_appealhir", rob_appealhi, Low)
        .game_acmd("game_appealhil", rob_appealhi, Low)

        //.game_acmd("game_appealsr", rob_appeals, Low)
        //.game_acmd("game_appealsl", rob_appeals, Low)
        
        .install();
}