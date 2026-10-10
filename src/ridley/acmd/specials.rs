use super::*;

//-------------------SPECIALS------------------

// UP SPECIAL
unsafe extern "C" fn ridley_specialairhi(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 1.0);
}

// UP SPECIAL EFFECT
unsafe extern "C" fn ridley_effect_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ridley_charge_hold"), Hash40::new("top"), 2, 0, -1, 0, 0, 0, 1, true);
    }
}

// UP SPECIAL SOUND
unsafe extern "C" fn ridley_sound_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_h01"));
    }
}

// UP SPECIAL EXPRESSION
unsafe extern "C" fn ridley_expression_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_loopattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// UP SPECIAL CHARGE
unsafe extern "C" fn ridley_specialairhicharge(agent: &mut L2CAgentBase) {
    if AURA[get_entry_id(agent.module_accessor)] { 
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 40, 10, 0, 25, 10.0, 0.0, 10.0, -18.0, Some(0.0), Some(10.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.2, 60, 70, 0, 65, 10.0, 0.0, 10.0, -18.0, Some(0.0), Some(10.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

// UP SPECIAL CHARGE EFFECT
unsafe extern "C" fn ridley_effect_specialairhicharge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ridley_charge"), Hash40::new("rot"), 0, 0, 21, 90, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

// UP SPECIAL CHARGE SOUND
unsafe extern "C" fn ridley_sound_specialairhicharge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_ridley_special_h02"));
    }
}

// UP SPECIAL CHARGE EXPRESSION
unsafe extern "C" fn ridley_expression_specialairhicharge(agent: &mut L2CAgentBase) {
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
        .game_acmd("game_specialairhi", ridley_specialairhi, Low)
        .effect_acmd("effect_specialairhi", ridley_effect_specialairhi, Low)
        .sound_acmd("sound_specialairhi", ridley_sound_specialairhi, Low)
        .expression_acmd("expression_specialairhi", ridley_expression_specialairhi, Low)

        .game_acmd("game_specialairhicharge", ridley_specialairhicharge, Low)
        .effect_acmd("effect_specialairhicharge", ridley_effect_specialairhicharge, Low)
        .sound_acmd("sound_specialairhicharge", ridley_sound_specialairhicharge, Low)
        .expression_acmd("expression_specialairhicharge", ridley_expression_specialairhicharge, Low)

        .install();
}