use super::*;

//FOOTSTOOL
unsafe extern "C" fn mario_stepjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 5.0, 270, 80, 0, 35, 4.2, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 5.0, 270, 80, 0, 35, 4.2, -1.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//SHRINK
unsafe extern "C" fn mario_shrink(agent: &mut L2CAgentBase) {
    let posx = PostureModule::pos_x(agent.module_accessor);
    let posy = PostureModule::pos_y(agent.module_accessor);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        CameraModule::reset_all(agent.module_accessor);
        macros::CAM_ZOOM_IN_arg5(agent, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        //StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SMALL, true);
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_MUSHD), 0, 0, false, false);
    }
    frame(agent.lua_state_agent, 11.0);
    ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
    for _ in 0..9 {
        if macros::is_excute(agent) {
            PostureModule::set_pos_2d(agent.module_accessor, &Vector2f {x: posx, y: posy});
            wait(agent.lua_state_agent, 1.0);
        }
    }
    wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        CameraModule::reset_all(agent.module_accessor);
        macros::CAM_ZOOM_OUT(agent);
    }
}


pub fn install() {
    Agent::new("mario")
        .game_acmd("game_stepjump", mario_stepjump, Low)

        .game_acmd("game_shrink", mario_shrink, Low)

        .install();
}