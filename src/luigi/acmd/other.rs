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
            macros::FT_MOTION_RATE(agent, 100.0);
        }
        
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR { 
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 270, 50, 0, 100, 7.2, 4.2, 3.0, -1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }  
    }
}

//------------------SHIELD------------------

// GUARD ON
unsafe extern "C" fn luigi_guardon(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
        let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_timer"), Hash40::new("hip"), &dumb, &dumb, 1.2, &dumb, &dumb, false, 0, 0, 0);
        EffectModule::set_rgb(agent.module_accessor, eff.try_into().unwrap(), 0.0, 1.0, 0.5);
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

        .game_acmd("game_guardon", luigi_guardon, Low)
        
        .install();
}