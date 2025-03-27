use super::*;

//-----------------SMASH ATTACKS----------------

// FORWARD SMASH
unsafe extern "C" fn pit_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..10 {
        if macros::is_excute(agent) {
            macros::SET_SPEED_EX(agent, 1.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.7, 48, 100, 50, 0, 3.0, 0.0, 4.6, 0.8, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.7, 335, 100, 30, 0, 3.0, 0.0, 13.0, 1.2, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.7, 220, 100, 30, 0, 2.5, 0.0, 12.5, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.7, 140, 100, 30, 0, 2.5, 0.0, 12.5, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.7, 130, 100, 65, 0, 3.0, 0.0, 4.6, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 8.5, 361, 100, 0, 60, 9.5, 0.0, 9.5, 3.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// FORWARD SMASH EFFECT
unsafe extern "C" fn pit_effect_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_atk_air_n"), Hash40::new("swordr2"), 0, 0, 0, -90, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_sword"), Hash40::new("swordr2"), 0, -0.8, 0, 180, -90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_sword"), Hash40::new("swordr2"), 0, 0.8, 0, 0, 90, 0, 1, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_atk_air_n"), true, true);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pit_sword1"), Hash40::new("tex_pit_sword2"), 3, Hash40::new("swordr1"), 0, 1, -0.2, Hash40::new("swordr1"), 0, 11.4, -1.2, true, Hash40::new("null"), Hash40::new("swordr1"), 0, 0, 0, 0, 90, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_sword"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7, 8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

// FORWARD SMASH SOUND
unsafe extern "C" fn pit_sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pit_attackair_n01"));
        macros::PLAY_SE(agent, Hash40::new("vc_pit_attacks4"));
    }
    wait(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

// FORWARD SMASH EXPRESSION
unsafe extern "C" fn pit_expression_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("swordr1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("pit")
        .game_acmd("game_attacks4", pit_attacks4, Low)
        .effect_acmd("effect_attacks4", pit_effect_attacks4, Low)
        .sound_acmd("sound_attacks4", pit_sound_attacks4, Low)
        .expression_acmd("expression_attacks4", pit_expression_attacks4, Low)

        .install();
}