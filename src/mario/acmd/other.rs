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
    macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        CameraModule::reset_all(agent.module_accessor);
        macros::CAM_ZOOM_IN_arg5(agent, /*frames*/ 200.0,/*no*/ 0.0,/*zoom*/ 1.0,/*yrot*/ 0.0,/*xrot*/ 0.0);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        PostureModule::set_scale(agent.module_accessor, curr_scale - 0.4, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
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

// SIDE TAUNT
unsafe extern "C" fn mario_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("block"), true);
        ItemModule::remove_item(agent.module_accessor, 0);
    }
    let rand = smash::app::sv_math::rand(hash40("agent"), 20) as u64;
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) { 
        if rand == 1 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_SUPERSTAR), 0, 0, false, false);
            ItemModule::throw_item(agent.module_accessor, 90.0, 0.5, 0.0, 0, true, 0.0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else if rand == 2 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_MUSHROOM), 0, 0, false, false);
            ItemModule::throw_item(agent.module_accessor, 90.0, 0.5, 0.0, 0, true, 0.0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else if rand == 3 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_MUSHD), 0, 0, false, false);
            ItemModule::throw_item(agent.module_accessor, 90.0, 0.5, 0.0, 0, true, 0.0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else if rand == 4 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_GREENSHELL), 0, 0, false, false);
            ItemModule::throw_item(agent.module_accessor, 90.0, 0.5, 0.0, 0, true, 0.0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            // increase coin counter
            macros::PLAY_SE(agent, Hash40::new("se_common_coin"));
            macros::EFFECT(agent, Hash40::new("sys_s_jump"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("block"), false);
    }
}

// SIDE TAUNT EFFECT
unsafe extern "C" fn mario_effect_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

// SIDE TAUNT SOUND
unsafe extern "C" fn mario_sound_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_jump01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_stage_mario_pastx_block_hit"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_stage_mario_pastx_hatenablock_hit"));
    }
}

// SIDE TAUNT EXPRESSION
unsafe extern "C" fn mario_expression_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("mario")
        .game_acmd("game_stepjump", mario_stepjump, Low)

        .game_acmd("game_shrink", mario_shrink, Low)

        .game_acmd("game_squat", mario_squat, Low)

        .game_acmd("game_appealsl", mario_appeals, Low)
        .game_acmd("game_appealsr", mario_appeals, Low)
        .effect_acmd("effect_appealsl", mario_effect_appeals, Low)
        .effect_acmd("effect_appealsr", mario_effect_appeals, Low)
        .sound_acmd("sound_appealsl", mario_sound_appeals, Low)
        .sound_acmd("sound_appealsr", mario_sound_appeals, Low)
        .expression_acmd("expression_appealsl", mario_expression_appeals, Low)
        .expression_acmd("expression_appealsr", mario_expression_appeals, Low)

        .install();
}