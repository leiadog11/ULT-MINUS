use super::*;


//FIREBALL
unsafe extern "C" fn mario_regular(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 5) as u64;
    if rand != 2 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 20, 0, 35, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 15, 0, 28, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 10, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_FIREBALL, *ATTACK_REGION_NONE);
        }
    }
    else {
        if macros::is_excute(agent) {
            let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
            let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_freezer"), Hash40::new("top"), &dumb, &dumb, 0.7, &dumb, &dumb, false, 0, 0, 0);
            EffectModule::set_rgb(agent.module_accessor, eff.try_into().unwrap(), 0.0, 0.8, 0.7);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
            let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_freezer"), Hash40::new("top"), &dumb, &dumb, 0.7, &dumb, &dumb, false, 0, 0, 0);
            EffectModule::set_rgb(agent.module_accessor, eff.try_into().unwrap(), 0.0, 0.8, 0.7);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 0, 50, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
            let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_freezer"), Hash40::new("top"), &dumb, &dumb, 0.7, &dumb, &dumb, false, 0, 0, 0);
            EffectModule::set_rgb(agent.module_accessor, eff.try_into().unwrap(), 0.0, 0.8, 0.7);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 0, 50, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
    }
}


pub fn install() {
    Agent::new("mario_fireball")
        .game_acmd("game_regular", mario_regular, Low)
        .install();
}