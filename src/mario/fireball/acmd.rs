use super::*;

// FIREBALL
unsafe extern "C" fn fireball_regular(agent: &mut L2CAgentBase) {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
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
            WorkModule::on_flag(owner_boma, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(owner_boma, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 0, 50, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 0, 50, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
    }
}

// FIREBALL EFFECT
unsafe extern "C" fn fireball_effect_regular(agent: &mut L2CAgentBase) {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::is_flag(owner_boma, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL) {
        if macros::is_excute(agent) {
            let dumb = Vector3f{x:0.0,y:0.0,z:0.0};
            let eff = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("sys_freezer"), Hash40::new("top"), &dumb, &dumb, 0.7, &dumb, &dumb, false, 0, 0, 0);
            EffectModule::set_rgb(agent.module_accessor, eff.try_into().unwrap(), 0.0, 0.8, 0.7);
        }
    }
    else {
         if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_bullet_l"), Hash40::new("rot"), 0, 1.8, 0, 0, 0, 0, 1, true);
        }
    }
}

pub fn install() {
    Agent::new("mario_fireball")
        .game_acmd("game_regular", fireball_regular, Low)
        .effect_acmd("effect_regular", fireball_effect_regular, Low)

        .install();
}