use super::*;

//-------------------ACMD------------------

// FLY
unsafe extern "C" fn bowarrow_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 80, 88, 0, 4, 1.3, 0.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.6, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("pit_bowarrow")
        .game_acmd("game_fly", bowarrow_fly, Low)
        .install();
}