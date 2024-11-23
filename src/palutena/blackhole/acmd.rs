use super::*;

// BLACKHOLE
unsafe extern "C" fn blackhole_blackhole(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 0.5);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 50, 0, 65.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 4, -0.1, 1000, 1, 0, 0, 110, 80);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, true);
    }
    wait(agent.lua_state_agent, 156.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

// BLACKHOLE EFFECT
unsafe extern "C" fn blackhole_effect_blackhole(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 0.5);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("palutena_final_blackhole"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_final_backlight"), false, false);
    }
}

pub fn install() {
    Agent::new("palutena_blackhole")
        .game_acmd("game_blackhole", blackhole_blackhole, Low)
        .effect_acmd("effect_blackhole", blackhole_effect_blackhole, Low)
        .install();
}