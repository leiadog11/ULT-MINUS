use super::*;

//-------------------SPECIALS------------------

// UP SPECIAL START
unsafe extern "C" fn ridley_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 40, 10, 0, 25, 15.0, 0.0, 10.0, -10.0, None, None, Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.2, 60, 70, 0, 65, 15.0, 0.0, 10.0, -10.0, None, None, Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP SPECIAL START EFFECT
unsafe extern "C" fn ridley_effect_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ridley_charge"), Hash40::new("rot"), 0, -7, 21, 110, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

// UP SPECIAL START SOUND
unsafe extern "C" fn ridley_sound_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_ridley_special_h02"));
    }
}

// UP SPECIAL START EXPRESSION
unsafe extern "C" fn ridley_expression_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_fly"), 12, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_fly"), 12, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_specialhistart", ridley_specialhistart, Low)
        .effect_acmd("effect_specialhistart", ridley_effect_specialhistart, Low)
        .sound_acmd("sound_specialhistart", ridley_sound_specialhistart, Low)
        .expression_acmd("expression_specialhistart", ridley_expression_specialhistart, Low)

        .install();
}