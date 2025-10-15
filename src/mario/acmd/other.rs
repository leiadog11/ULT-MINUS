use super::*;

//-----------------OTHER----------------

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
    let mut curr_scale = PostureModule::scale(agent.module_accessor);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        CameraModule::reset_all(agent.module_accessor);
        macros::CAM_ZOOM_IN_arg5(agent, /*frames*/ 5.0,/*no*/ 0.0,/*zoom*/ 3.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        SlowModule::set_whole(agent.module_accessor, 100, 5);
        macros::PLAY_SE(agent, Hash40::new("se_item_mushd"));
        PostureModule::set_scale(agent.module_accessor, curr_scale - 0.3, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        SlowModule::clear_whole(agent.module_accessor);
        CameraModule::reset_all(agent.module_accessor);
        macros::SET_SPEED_EX(agent, 1.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::CAM_ZOOM_OUT(agent);
    }
}

// CROUCH
unsafe extern "C" fn mario_squat(agent: &mut L2CAgentBase) {
    let x_vel = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        sv_kinetic_energy::friction_off(agent.lua_state_agent);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        if x_vel > 0.0 || x_vel < 0.0 {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: (x_vel * 1.1) * lr, y: 0.0, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
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
            ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else if rand == 2 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_MUSHROOM), 0, 0, false, false);
            ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else if rand == 3 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_MUSHD), 0, 0, false, false);
            ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else if rand == 4 {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_GREENSHELL), 0, 0, false, false);
            ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
            macros::EFFECT(agent, Hash40::new("sys_item_arrival"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            macros::PLAY_SE(agent, Hash40::new("se_common_coin"));
            COIN_COUNT[get_entry_id(agent.module_accessor)] += 1;
            macros::EFFECT(agent, Hash40::new("sys_s_jump"), Hash40::new("block"), 1.0, 0, 0, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);
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