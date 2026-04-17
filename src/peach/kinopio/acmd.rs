use super::*;

//--------------------KINOPIO-----------------------

// APPEAR
unsafe extern "C" fn kinopio_appear(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// APPEAR EFFECT
unsafe extern "C" fn kinopio_effect_appear(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// APPEAR SOUND
unsafe extern "C" fn kinopio_sound_appear(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// WAIT
unsafe extern "C" fn kinopio_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// WAIT EFFECT
unsafe extern "C" fn kinopio_effect_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// WAIT SOUND
unsafe extern "C" fn kinopio_sound_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// PUNCH
unsafe extern "C" fn kinopio_punch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 30, 100, 70, 0, 5.5, 0.0, 5.0, 7.8, None, None, None, 2.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
}

// PUNCH EFFECT
unsafe extern "C" fn kinopio_effect_punch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -3.5, 4.2, 0, 0, 0, 0, 0.65, 0, 1, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 9.8, 4.5, 0, 0, 0, 0, 0.6, 0, 1, 1, 0, 0, 360, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
}

// PUNCH SOUND
unsafe extern "C" fn kinopio_sound_punch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_attackair_f01"));
    }
}

// SPRAY
unsafe extern "C" fn kinopio_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) { 
       AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    let rand = smash::app::sv_math::rand(hash40("agent"), 13) as u64;
    if rand < 3 { 
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        } 
    } else {
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        }
    }
}

// SPRAY EFFECT
unsafe extern "C" fn kinopio_effect_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("peach_kinopio_hit"), Hash40::new("top"), 0, 6.5, 9.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("peach_kinopio_hit"), false, false);
        macros::COL_NORMAL(agent);
    }
}

// SPRAY SOUND
unsafe extern "C" fn kinopio_sound_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_peach_special_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n02"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n04"));
    }
}

// AIR SPRAY
unsafe extern "C" fn kinopio_air_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) { 
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    let rand = smash::app::sv_math::rand(hash40("agent"), 13) as u64;
    if rand < 3 { 
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        } 
    } else {
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 45, 150, 0, 40, 10.5, 0.0, 5.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        }
    }
}

// AIR SPRAY EFFECT
unsafe extern "C" fn kinopio_effect_air_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("peach_kinopio_hit"), Hash40::new("top"), 0, 6.5, 13, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("peach_kinopio_hit"), false, false);
        macros::COL_NORMAL(agent);
    }
}

// AIR SPRAY SOUND
unsafe extern "C" fn kinopio_sound_air_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_peach_special_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n02"));
    }
}

// FALL
unsafe extern "C" fn kinopio_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 270, 120, 80, 0, 6.0, 0.0, 13.0, 0.0, None, None, None, 2.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
}

// FALL EFFECT
unsafe extern "C" fn kinopio_effect_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flyroll_smoke"), Hash40::new("top"), 0, 16, 0, 90, 0, 0, 0.3, true);
    }
}

// FALL SOUND
unsafe extern "C" fn kinopio_sound_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_catch_kinopio"));
    }
}

// LAND
unsafe extern "C" fn kinopio_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        
    }
}

// LAND EFFECT
unsafe extern "C" fn kinopio_effect_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

// LAND SOUND
unsafe extern "C" fn kinopio_sound_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_catch_kinopio"));
    }
}

pub fn install() {
    Agent::new("peach_kinopio")
        .game_acmd("game_appear", kinopio_appear, Low)
        .effect_acmd("effect_appear", kinopio_effect_appear, Low)
        .sound_acmd("sound_appear", kinopio_sound_appear, Low)

        .game_acmd("game_wait", kinopio_wait, Low)
        .effect_acmd("effect_wait", kinopio_effect_wait, Low)
        .sound_acmd("sound_wait", kinopio_sound_wait, Low)

        .game_acmd("game_punch", kinopio_punch, Low)
        .effect_acmd("effect_punch", kinopio_effect_punch, Low)
        .sound_acmd("sound_punch", kinopio_sound_punch, Low)

        .game_acmd("game_spray", kinopio_spray, Low)
        .effect_acmd("effect_spray", kinopio_effect_spray, Low)
        .sound_acmd("sound_spray", kinopio_sound_spray, Low)

        .game_acmd("game_air_spray", kinopio_air_spray, Low)
        .effect_acmd("effect_air_spray", kinopio_effect_air_spray, Low)
        .sound_acmd("sound_air_spray", kinopio_sound_air_spray, Low)

        .game_acmd("game_fall", kinopio_fall, Low)
        .effect_acmd("effect_fall", kinopio_effect_fall, Low)
        .sound_acmd("sound_fall", kinopio_sound_fall, Low)

        .game_acmd("game_land", kinopio_land, Low)
        .effect_acmd("effect_land", kinopio_effect_land, Low)
        .sound_acmd("sound_land", kinopio_sound_land, Low)
        
        .install();
}