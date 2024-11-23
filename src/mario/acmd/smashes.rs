use super::*;

//-----------------SMASH ATTACKS----------------

//UP SMASH
unsafe extern "C" fn mario_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    execute(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 14.0, 83, 94, 0, 32, 5.0, 2.5, 1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 14.0, 83, 94, 0, 32, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("mario")
        .game_acmd("game_attackhi4", mario_attackhi4, Low)
        
        .install();
}