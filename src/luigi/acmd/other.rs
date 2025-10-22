use super::*;

//-----------------TAUNT-----------------

// DOWN TAUNT
unsafe extern "C" fn luigi_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 270, 100, 180, 0, 3.0, 0.0, -1.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP TAUNT
unsafe extern "C" fn luigi_appealhi(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.2);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        
    }
}

// SIDE TAUNT
unsafe extern "C" fn luigi_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        if true {
            MotionModule::set_rate(agent.module_accessor, 0.0);
        }
        
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR { 
            macros::FT_MOTION_RATE(agent, 0.9);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 270, 50, 0, 100, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }  
    }
}

//------------------SHIELD------------------

// GUARD ON EFFECT
unsafe extern "C" fn luigi_effect_guardon(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        
    }
}

// GUARD ON SOUND
unsafe extern "C" fn luigi_sound_guardon(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_negative_zone"));
    }
}

// GUARD OFF EFFECT
unsafe extern "C" fn luigi_effect_guardoff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_negative_zone"), false, false);
    }
}

// GUARD OFF SOUND
unsafe extern "C" fn luigi_sound_guardoff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_luigi_negative_zone"));
    }
}

// CROUCH
unsafe extern "C" fn luigi_squat(agent: &mut L2CAgentBase) {
    let x_vel = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        sv_kinetic_energy::friction_off(agent.lua_state_agent);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        if x_vel > 0.0 || x_vel < 0.0 {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: (x_vel + 2.5) * lr, y: 0.0, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_appeallwl", luigi_appeallw, Low)
        .game_acmd("game_appeallwr", luigi_appeallw, Low)

        .game_acmd("game_appealhir", luigi_appealhi, Low)
        .game_acmd("game_appealhil", luigi_appealhi, Low)

        .game_acmd("game_appealsr", luigi_appeals, Low)
        .game_acmd("game_appealsl", luigi_appeals, Low)

        .effect_acmd("effect_guardon", luigi_effect_guardon, Low)
        .sound_acmd("sound_guardon", luigi_sound_guardon, Low)

        .effect_acmd("effect_guardoff", luigi_effect_guardoff, Low)
        .sound_acmd("sound_guardoff", luigi_sound_guardoff, Low)

        .game_acmd("game_squat", luigi_squat, Low)
        
        .install();
}