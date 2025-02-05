use super::*;

// FOOTSTOOL
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

// SHRINK
unsafe extern "C" fn mario_shrink(agent: &mut L2CAgentBase) {
    let x_vel = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_vel = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut curr_scale = PostureModule::scale(agent.module_accessor);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, (x_vel * 0.0), (y_vel * 0.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        CameraModule::reset_all(agent.module_accessor);
        macros::CAM_ZOOM_IN_arg5(agent, /*frames*/ 10.0,/*no*/ 0.0,/*zoom*/ 1.0,/*yrot*/ 0.0,/*xrot*/ 0.0);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        PostureModule::set_scale(agent.module_accessor, curr_scale - 0.5, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        CameraModule::reset_all(agent.module_accessor);
        macros::CAM_ZOOM_OUT(agent);
    }
}

// CROUCH
unsafe extern "C" fn mario_squat(agent: &mut L2CAgentBase) {
    let x_vel = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        if x_vel > 0.0 {
            macros::SET_SPEED_EX(agent, (x_vel * lr) + 0.2, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            macros::SET_SPEED_EX(agent, (x_vel * lr), 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

pub fn install() {
    Agent::new("mario")
        .game_acmd("game_stepjump", mario_stepjump, Low)

        .game_acmd("game_shrink", mario_shrink, Low)

        .game_acmd("game_squat", mario_squat, Low)

        .install();
}