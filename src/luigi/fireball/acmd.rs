use super::*;

// FIREBALL
unsafe extern "C" fn fireball_regular(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("fighter"), 7) as u64;
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);
    if rand != 1 {
        MISFIRE_SPECIAL_N[ENTRY_ID] = false;
        if macros::is_excute(agent) {
            macros::FT_MOTION_RATE(agent, 2.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 25, 0, 7, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 25, 0, 7, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    else {
        MISFIRE_SPECIAL_N[ENTRY_ID] = true;
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03")); 
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 88, 88, 0, 50, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("luigi_fireball")
        .game_acmd("game_regular", fireball_regular, Low)
        .install();
}