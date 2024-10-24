use super::*;

/*
// THUNDER - REGULAR
unsafe extern "C" fn kaminari_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PostureModule::set_rot(agent.module_accessor, &Vector3f{x: 0.0, y: 90.0, z: 0.0}, 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 270, 60, 125, 55, 7.0, 0.0, 2.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 94, 60, 0, 74, 4.0, 0.0, 4.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}
*/

pub fn install() {
    /*
    Agent::new("pichu_kaminari")
        .game_acmd("game_regular", kaminari_regular, Low)
        .install();
    */
}