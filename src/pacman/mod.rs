
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*,
    smashline::Priority::*
};

use smashline::Main;
use std::convert::TryInto;

pub const SITUATION_KIND:                  i32 = 0x16;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH : i32 = 0x200000EC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE : i32 = 0x100000C9;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN : i32 = 0x100000CA;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN : i32 = 0x100000BD;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN : i32 = 0x100000BC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN : i32 = 0x100000BB;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN : i32 = 0x100000BA;

//OPFF
unsafe extern "C" fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("ghost1"), &Vector3f{x: 0.0, y: -90.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("ghost2"), &Vector3f{x: 0.0, y: -90.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN) > 0 {
                WorkModule::dec_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN);
        }
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN);
        }
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN);
        }
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN);
        }
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN);
        }
    }
}

//ON START
unsafe extern "C" fn pacman_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), false);
    }
}



// -------GROUND-------

//JAB
unsafe extern "C" fn pacman_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.5, 0.0, 7.5, 5.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 20, 1.7, 0.0, 7.5, 8.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 180, 15, 0, 20, 2.0, 0.0, 7.5, 11.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 2.0, 0.0, 7.5, 11.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 4.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}
unsafe extern "C" fn pacman_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 35, 0, 35, 2.5, 0.0, 7.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 35, 0, 30, 2.8, 0.0, 7.5, 4.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 30, 0, 30, 3.0, 0.0, 7.5, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}
unsafe extern "C" fn pacman_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 106, 0, 45, 4.0, 0.0, 8.0, 7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 106, 0, 45, 5.0, 0.0, 8.0, 13.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        CancelModule::enable_cancel(agent.module_accessor);
        AttackModule::clear_all(agent.module_accessor);
    }
}


// DASH ATTACK
unsafe extern "C" fn pacman_attackdash(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("pizzapacman"), 0.0, 361, 100, 12, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 7.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.8, 361, 15, 0, 45, 7.0, 0.0, 5.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
        for _ in 0..3 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.8, 361, 15, 0, 45, 7.0, 0.0, 5.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            }
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                AttackModule::clear_all(agent.module_accessor);
            }
            wait(agent.lua_state_agent, 5.0);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 4.0, 60, 65, 0, 100, 7.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// FORWARD TILT
unsafe extern "C" fn pacman_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 8.0, 120, 80, 0, 25, 4.8, 7.0, -1.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 120, 80, 0, 25, 3.6, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 8.0, 361, 100, 0, 20, 3.0, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//DAFT
unsafe extern "C" fn pacman_attacks3lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 6.0, 361, 1, 1, 1, 4.8, 6.5, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 361, 1, 1, 1, 3.6, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 6.0, 361, 1, 1, 1, 3.0, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TILT
unsafe extern "C" fn pacman_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.4);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 330, 55, 0, 25, 6.0, 0.0, 3.6, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.75, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP TILT
unsafe extern "C" fn pacman_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 6.5, 105, 68, 0, 43, 6.5, 4.2, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 6.5, 93, 68, 0, 43, 5.5, 2.9, -0.5, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 6.5, 93, 68, 0, 43, 3.2, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_OFF);
    }
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// FORWARD SMASH
unsafe extern "C" fn pacman_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 16.0, 361, 97, 0, 40, 5.4, 0.0, 5.4, 0.0, Some(0.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 97, 0, 40, 3.0, 0.0, 6.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 9.0, 361, 102, 0, 40, 4.3, 0.0, 5.4, 0.0, Some(0.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, false, -1);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


// DOWN SMASH
unsafe extern "C" fn pacman_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 13.0, 29, 86, 0, 30, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("ghost2"), 13.0, 29, 86, 0, 30, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 7.0, 45, 92, 0, 40, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(1.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("ghost2"), 7.0, 45, 92, 0, 40, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(1.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, false, -1);
    }
    wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//UP SMASH
unsafe extern "C" fn pacman_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH);
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 135, 100, 125, 0, 3.5, 0.0, 8.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 14.0, 83, 97, 0, 32, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 8.0, 83, 92, 0, 32, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, false, -1);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH);
        AttackModule::clear_all(agent.module_accessor);
    }
}

// ------AERIALS------

// DOWN AIR
unsafe extern "C" fn pacman_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 4.0);
    }
    frame(agent.lua_state_agent, 6.0);
    for _ in 0..3 {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 245, 100, 5, 0, 4.5, 0.0, 1.8, 0.5, Some(0.0), Some(4.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 130, 100, 22, 0, 6.2, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 100, 100, 35, 0, 6.2, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 5.0);
}
if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 270, 55, 0, 55, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
}
wait(agent.lua_state_agent, 2.0);
if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
}
frame(agent.lua_state_agent, 50.0);
if macros::is_excute(agent) {
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}

// NAIR
unsafe extern "C" fn pacman_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    if macros::is_excute(agent) {
        for _ in 0..10 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 0.6, 94, 25, 0, 10, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            wait(agent.lua_state_agent, 3.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 5.2, 60, 40, 0, 60, 10.5, 0.0, 0.7, 0.0, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// FORWARD AIR
unsafe extern "C" fn pacman_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 7.65, 60, 29, 0, 66, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 7.65, 60, 29, 0, 66, 5.0, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

// UP AIR
unsafe extern "C" fn pacman_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("legr"), 6.4, 0, 12, 19, 0, 0, 0, 1.35, 1.5, 135, false, 5.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 70, 100, 0, 13, 6.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 70, 100, 0, 13, 7.5, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 315, 100, 0, 13, 6.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 315, 100, 0, 13, 7.5, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


// ------SPECIALS------

//NEUTRAL B

//N - PRE
unsafe extern "C" fn pacman_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_ATTR_START_TURN,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    return 0.into();
}

//N - MAIN
unsafe extern "C" fn pacman_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::PLAY_SE(fighter, Hash40::new("se_pacman_special_N04"));

    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false); 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
      
      fighter.fastshift(L2CValue::Ptr(pacman_specialn_main_loop as *const () as _ ));
      return 0.into()
}

//N - MAIN LOOP
unsafe extern "C" fn pacman_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }  

        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, true); 
        fighter.fastshift(L2CValue::Ptr(pacman_specialn_end as *const () as _ ));
        return 0.into()
    }

    return 0.into();
}

//N - END
unsafe extern "C" fn pacman_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//HOLD - PRE
unsafe extern "C" fn pacman_specialn_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_ATTR_START_TURN,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    return 0.into();
}

//HOLD - MAIN
unsafe extern "C" fn pacman_specialn_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), true);
    let item_choice = 0;
    WorkModule::set_int(fighter.module_accessor, item_choice, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);

    fighter.fastshift(L2CValue::Ptr(pacman_specialn_hold_main_loop as *const () as _ ));
    return 0.into()
}

//HOLD - MAIN LOOP
unsafe extern "C" fn pacman_specialn_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let xpos = ControlModule::get_stick_x(fighter.module_accessor);
    let ypos = ControlModule::get_stick_y(fighter.module_accessor);
    let selected_scale = Vector3f{x: 1.5, y: 1.5, z: 1.0};
    let cooldown_scale = Vector3f{x: 0.3, y: 0.3, z: 1.0};
    let facing = PostureModule::lr(fighter.module_accessor);
    /*
    if facing == 1.0 {
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("wheel"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    else {
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("wheel"), &Vector3f{x: 0.0, y: 180.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    */

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
            }   
        }

        //key
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN) != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), false);
        }
        else {
            if xpos == 0.0 && ypos > 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("key"), &selected_scale);
                macros::PLAY_SE(fighter, Hash40::new("se_pacman_special_N08"));
                WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);
            }
        }

        //apple
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN) != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), false);
        }
        else {
            if xpos > 0.0 && ypos > 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("apple"), &selected_scale);
                macros::PLAY_SE(fighter, Hash40::new("se_pacman_special_N04"));
                WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);
            }
        }

        //melon
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN) != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), false);       
        }
        else {
            if xpos > 0.0 && ypos < 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("melon"), &selected_scale);
                macros::PLAY_SE(fighter, Hash40::new("se_pacman_special_N05"));
                WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);
            }
        }

        //galaxian
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN) != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), false);       
        }
        else {
            if xpos < 0.0 && ypos < 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("galaxian"), &selected_scale);
                macros::PLAY_SE(fighter, Hash40::new("se_pacman_special_N06"));
                WorkModule::set_int(fighter.module_accessor, 4, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);
            }
        }

        //bell
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN) != 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), false);        
        }
        else {
            if xpos < 0.0 && ypos > 0.0 {
                ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("bell"), &selected_scale);
                macros::PLAY_SE(fighter, Hash40::new("se_pacman_special_N07"));
                WorkModule::set_int(fighter.module_accessor, 5, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);
            }
        }

        if xpos == 0.0 && ypos == 0.0 {
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE);
        }
    }
    else {
        //key
        if  WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE) == 1 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);
            WorkModule::set_int(fighter.module_accessor, 180, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN);
        }

        //apple
        else if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE) == 2 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
            WorkModule::set_int(fighter.module_accessor, 60, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN);
        }

        //melon
        else if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE) == 3 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
            WorkModule::set_int(fighter.module_accessor, 60, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN);
        }

        //galaxian
        else if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE) == 4 {
            let rand = smash::app::sv_math::rand(hash40("fighter"), 9) as u64;
            if rand != 8 {
                ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
                WorkModule::set_int(fighter.module_accessor, 90, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN);
            }
            else {
                ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BOSSGALAGA), 0, 0, false, false);
                WorkModule::set_int(fighter.module_accessor, 90, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN);
            }
        }

        //bell
        else if WorkModule::get_int(fighter.module_accessor, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE) == 5 {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
            WorkModule::set_int(fighter.module_accessor, 180, FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN);
        }

        else {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
                //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
                fighter.change_status((*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL).into(), false.into());
                return 1.into()
            }
            else {
                //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
                fighter.change_status((*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL).into(), false.into());
                return 1.into()
            }   
        }

        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
            return 1.into();
        }   
    }

    return 0.into()
}

//HOLD - END
unsafe extern "C" fn pacman_specialn_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), false);

    return 0.into();
}


// DOWN B
unsafe extern "C" fn hydrant_fall(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 10, 5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
}


// SIDE B
unsafe extern "C" fn pacman_specialsdash(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 50, 108, 0, 50, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            wait(agent.lua_state_agent, 6.0);
            if smash::app::stage::get_stage_id() == 0x0 {
            //Battlefield
            if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x3 {
            //FD
            if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x15B {
            //Small Battlefield
            if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x5F {
            //Smashville
            if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 140.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 140.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x6B {
            //PS2
            if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0xF2 {
            //Kalos
            if PostureModule::pos_x(agent.module_accessor) >= 165.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -165.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x101 {
            //Town and City
            if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 105.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 105.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else if smash::app::stage::get_stage_id() == 0x169 {
            //Hollow Bastion
            if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
            else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
                PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
                AttackModule::clear_all(agent.module_accessor);
                GroundModule::set_collidable(agent.module_accessor, false);
            }
        }
        else{
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 361, 66, 0, 40, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            GroundModule::set_collidable(agent.module_accessor, true);
        }
        
}
}


// ------THROWS------

// GRAB
unsafe extern "C" fn pacman_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 4.0);
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//GRAB EFFECT
unsafe extern "C" fn pacman_effect_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        for _ in 0..600 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("pacman_tractorbeam"), Hash40::new("top"), 0, 7, 11, -90, 0, 90, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE, 0, 0);
            EffectModule::set_custom_uv_offset(agent.module_accessor, (*FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE).try_into().unwrap(), std::ptr::null(), 0);
            wait(agent.lua_state_agent, 60.0);
        }
    }
}

//DASH GRAB
unsafe extern "C" fn pacman_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 4.0);
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.5, Some(0.0), Some(-2.5), Some(0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 1.5, Some(0.0), Some(-2.5), Some(1.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 2.0, Some(0.0), Some(-2.5), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//GRAB SOUND
unsafe extern "C" fn pacman_sound_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pacman_capture"));
    }
    frame(agent.lua_state_agent, 154.0);
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

// UP THROW
unsafe extern "C" fn pacman_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 89, 100, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 0, 22);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, -1);
    }
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 0.842);
}

// DOWN THROW
unsafe extern "C" fn pacman_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_LEAVE_NEAR_OTTOTTO(agent, -2.5, 2.5);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 300, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 300, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..2 {
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 300, 100, 40, 40, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
wait(agent.lua_state_agent, 6.0);
if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 300, 100, 40, 40, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    AttackModule::set_catch_only_all(agent.module_accessor, true, false);
}
wait(agent.lua_state_agent, 1.0);
if macros::is_excute(agent) {
    macros::CHECK_FINISH_CAMERA(agent, 0, 0);
    lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.4);
}
wait(agent.lua_state_agent, 1.0);
if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
    let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
}
}

// BACK THROW
unsafe extern "C" fn pacman_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 45, 82, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(agent, 14, 3);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.6);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// FORWARD THROW
unsafe extern "C" fn pacman_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
	    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 58, 65, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
	    macros::CHECK_FINISH_CAMERA(agent, 23, 4);
	    lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
	    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// -----------TAUNTS---------------

// SIDE TAUNT
unsafe extern "C" fn pacman_appeal_side(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 86.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TAUNT
unsafe extern "C" fn pacman_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLEEP, false.into());
    }
}
unsafe extern "C" fn pacman_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLEEP, false.into());
    }
}

//---------PROJECTILES------------

//BIGPACMAN - GHOST
unsafe extern "C" fn bigpacman_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 92, 175, 0, 25, 8.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pacman_appeal_up"), Hash40::new("ghost1"), 0, 5, 0, 0, 0, 0, 1, true);
        for _ in 0..100 {
            macros::PLAY_SE(agent, Hash40::new("se_pacman_appeal_monster_ijike"));
            wait(agent.lua_state_agent, 30.0);
        }
    }
}


//STATUS
pub unsafe extern "C" fn pacman_bigpacman_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Snap to throw position
    let mut owner_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut article_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut offset_add = Vector3f{x:20.0,y:0.0,z:0.0};

    if WorkModule::is_flag(owner, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH) {
        offset_add = Vector3f{x:0.0,y:20.0,z:0.0};
    }

    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("throw")}, &mut owner_pos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut article_pos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + owner_pos.x - article_pos.x + (offset_add.x*lr), y: PostureModule::pos_y(owner) + owner_pos.y - (article_pos.y)+ offset_add.y, z: PostureModule::pos_z(owner) + owner_pos.z - article_pos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    0.into()
}

unsafe extern "C" fn pacman_bigpacman_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

unsafe extern "C" fn pacman_bigpacman_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    //Life
    let life = 200;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("start"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let facing = PostureModule::lr(weapon.module_accessor);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);

    if WorkModule::is_flag(owner_boma, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH) {
        speed_x = 0.0;
        speed_y = 1.0;
    }
    else { 
        speed_x = if facing == 1.0 { 1.0 } else { -1.0 };
        speed_y = 0.0; 
    }

    // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();
    
    // Set speed
    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    weapon.fastshift(L2CValue::Ptr(pacman_bigpacman_start_main_loop as *const () as _))
}
unsafe extern "C" fn pacman_bigpacman_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        bigpacman_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn bigpacman_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}


//TRAMPOLINE - PRE
unsafe extern "C" fn pacman_trampoline_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

//TRAMPOLINE - MAIN
unsafe extern "C" fn pacman_trampoline_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(0x327ed09d4f));
    sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
    weapon.pop_lua_stack(1);

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_DEAD_AREA) {
        let trampoline_life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_trampoline"), hash40("life"));
        WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_INT_RIDER_NUM); 
    }
    else {
        WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_INT_RIDER_NUM); 
    }
    WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_AREA_BODY);
    let area_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_trampoline"), hash40("area_frame"));
    WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_INT_AREA_BODY_FRAME);
    weapon.fastshift(L2CValue::Ptr(pacman_trampoline_start_main_loop as *const () as _ ))
}

//TRAMPOLINE - MAIN LOOP
unsafe extern "C" fn pacman_trampoline_start_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.change_status((*WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_SHAKE).into(), false.into());
    return 1.into();
}


pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_attack11", pacman_attack11, Low)
        .game_acmd("game_attack12", pacman_attack12, Low)
        .game_acmd("game_attack13", pacman_attack13, Low)
        .game_acmd("game_attacks3", pacman_attacks3, Low)
        .game_acmd("game_attacks3lw", pacman_attacks3lw, Low)
        .game_acmd("game_attacklw3", pacman_attacklw3, Low)
        .game_acmd("game_attackhi3", pacman_attackhi3, Low)
        .game_acmd("game_attacks4", pacman_attacks4, Low)
        .game_acmd("game_attacklw4", pacman_attacklw4, Low)
        .game_acmd("game_attackhi4", pacman_attackhi4, Low)
        .game_acmd("game_attackairlw", pacman_attackairlw, Low)
        .game_acmd("game_attackairn", pacman_attackairn, Low)
        .game_acmd("game_attackairf", pacman_attackairf, Low)
        .game_acmd("game_attackairhi", pacman_attackairhi, Low)
        .game_acmd("game_specialsdash", pacman_specialsdash, Low)
        .game_acmd("game_catch", pacman_catch, Low)
        .game_acmd("game_catchdash", pacman_catchdash, Low)
        .effect_acmd("effect_catch", pacman_effect_catch, Low)
        .sound_acmd("sound_catch", pacman_sound_catch, Low)
        .game_acmd("game_throwhi", pacman_throwhi, Low)
        .game_acmd("game_throwlw", pacman_throwlw, Low)
        .game_acmd("game_throwb", pacman_throwb, Low)
        .game_acmd("game_throwf", pacman_throwf, Low)
        .game_acmd("game_appealsr", pacman_appeal_side, Low)
        .game_acmd("game_appealsl", pacman_appeal_side, Low)
        .game_acmd("game_appeallwr", pacman_appeallwr, Low)
        .game_acmd("game_appeallwl", pacman_appeallwl, Low)
        .game_acmd("game_attackdash", pacman_attackdash, Low)
        .on_line(Main, pacman_frame)
        .on_start(pacman_start)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, pacman_specialn_end)
        .status(Pre, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, pacman_specialn_hold_pre)
        .status(Main, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, pacman_specialn_hold_main)
        .status(End, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD, pacman_specialn_hold_end)
        .install();
    Agent::new("pacman_firehydrant")
        .game_acmd("game_fall", hydrant_fall, Low)
        .install();
    Agent::new("pacman_bigpacman")
        .game_acmd("game_start", bigpacman_start, Low)
        .status(Init, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_START, pacman_bigpacman_start_init)
        .status(Pre, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_START, pacman_bigpacman_start_pre)
        .status(Main, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_START, pacman_bigpacman_start_main)
        .install();
    Agent::new("pacman_trampoline")
        .status(Pre, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_START, pacman_trampoline_start_pre)
        .status(Main, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_START, pacman_trampoline_start_main)
        .install();
}