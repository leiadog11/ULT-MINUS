use super::*;

// UP TILT
unsafe extern "C" fn rescue_attackhi3(agent: &mut L2CAgentBase) {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_posx = PostureModule::pos_x(owner_boma);
    let owner_posy = PostureModule::pos_y(owner_boma);
    let owner_posz = PostureModule::pos_z(owner_boma);

    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        PostureModule::set_pos(agent.module_accessor, &Vector3f{x: owner_posx + 5.5, y: owner_posy + 5.0, z: owner_posz});
        PostureModule::set_rot(agent.module_accessor, &Vector3f{x: 35.0, y: 0.0, z: 0.0}, 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 3.0, 80, 60, 0, 40, 5.0, 2.0, 3.0, 0.0, Some(2.0), Some(3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) { 
        PostureModule::set_pos(agent.module_accessor, &Vector3f{x: owner_posx + 0.5, y: owner_posy + 10.0, z: owner_posz});
        PostureModule::set_rot(agent.module_accessor, &Vector3f{x: -35.0, y: 0.0, z: 0.0}, 0);
        macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 7.0, 90, 80, 0, 60, 5.0, 2.0, 3.0, 0.0, Some(2.0), Some(3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("gamewatch_rescue")
        .game_acmd("game_attackhi3", rescue_attackhi3, Low)
        
        .install();
}