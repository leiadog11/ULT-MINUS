use super::*;

// PHANTOM KICK
unsafe extern "C" fn phantom_attackkick(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) { 
        VisibilityModule::set_whole(agent.module_accessor, true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("phantom_inner"), true);
        WorkModule::set_float(agent.module_accessor, 1.0, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_CHARGE_LEVEL);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 30, 0, 50, 5.5, 0.0, 6.0, 14.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("zelda_phantom")
        .game_acmd("game_attackkick", phantom_attackkick, Low)
        
        .install();
}