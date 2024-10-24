use super::*;

//---------------SPECIALS-----------------

// DOWN B
unsafe extern "C" fn rob_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_STATUS_GYRO_FLAG_SHOOT);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_ROBOT_GENERATE_ARTICLE_GYRO, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::set_int(agent.module_accessor, 300, FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE);
    }
}

// UP B
unsafe extern "C" fn rob_specialhi(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
    macros::SET_SPEED_EX(agent, 0.0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);
    }
}

//UP B EFFECT
unsafe extern "C" fn rob_effect_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("robot_roboburner_ring"), Hash40::new("top"), 0, 0, 0, 0, 90, -90, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_roboburner"), Hash40::new("knee"), 0, 0, 0, 0, 90, -90, 1, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

//NEUTRAL B
unsafe extern "C" fn rob_beam_flymax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 43, 0, 50, 3.5, 0.0, 0.0, 2.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 43, 0, 50, 3.5, 0.0, 0.0, 2.0, Some(0.0), Some(0.0), Some(-16.0), 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
    }
}

pub fn install() {
    Agent::new("robot")
        .game_acmd("game_speciallw", rob_speciallw, Low)
        .game_acmd("game_specialairlw", rob_speciallw, Low)

        .game_acmd("game_specialhi", rob_specialhi, Low)
        .effect_acmd("effect_specialhi", rob_effect_specialhi, Low)

        .install();
}