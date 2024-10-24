use super::*;

// GSWORD
unsafe extern "C" fn ganon_gsword_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 68, 20, 0, 35, 2.4, 0.0, 14.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 5, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

// GSWORD EFFECT
unsafe extern "C" fn ganon_gsword_effect_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_trace"), Hash40::new("haver"), 0, 1.5, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("tex_ganon_sword1"), Hash40::new("haver"), 0, 1.5, 0, 0, 0, 0, 1, true);
    }
}

pub fn install() {
    Agent::new("ganon_gsword")
        .game_acmd("game_regular", ganon_gsword_regular, Low)
        .effect_acmd("effect_regular", ganon_gsword_effect_regular, Low)
        .install();
}