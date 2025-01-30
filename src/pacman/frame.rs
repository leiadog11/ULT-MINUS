use super::*;

// OPFF
pub unsafe extern "C" fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);

        // CLIFF CAPE CHECK
        if situation_kind == *SITUATION_KIND_CLIFF || DamageModule::reaction(boma, 0) > 1.0 || status_kind == *FIGHTER_STATUS_KIND_DEAD { 
            ModelModule::set_mesh_visibility(boma, Hash40::new("cape"), true);
        }

        // ITEM COOLDOWNS
        if KEY_COOLDOWN[ENTRY_ID] > 0 {
            KEY_COOLDOWN[ENTRY_ID] -= 1; 
        }
        if APPLE_COOLDOWN[ENTRY_ID] > 0 {
            APPLE_COOLDOWN[ENTRY_ID] -= 1; 
        }
        if MELON_COOLDOWN[ENTRY_ID] > 0 {
            MELON_COOLDOWN[ENTRY_ID] -= 1; 
        }
        if GALAXIAN_COOLDOWN[ENTRY_ID] > 0 {
            GALAXIAN_COOLDOWN[ENTRY_ID] -= 1; 
        }
        if BELL_COOLDOWN[ENTRY_ID] > 0 {
            BELL_COOLDOWN[ENTRY_ID] -= 1; 
        }

        // NEUTRAL B MONADO WHEEL
        if status_kind != *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD {
            ModelModule::set_joint_rotate(boma, Hash40::new("ghost1"), &Vector3f{x: 0.0, y: -90.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(boma, Hash40::new("ghost2"), &Vector3f{x: 0.0, y: -90.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }

        // GHOST DIRECTIONS
        if motion != hash40("attack_s4") || motion != hash40("attack_lw4") || motion != hash40("attack_hi4") {
            DOWN_SMASH[ENTRY_ID] = false;
            UP_SMASH[ENTRY_ID] = false;
        } 
    }
}

// ON START
pub unsafe extern "C" fn pacman_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("key"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("apple"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("melon"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("galaxian"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("bell"), false);
    }
}

pub fn install() {
    Agent::new("pacman")
        .on_line(Main, pacman_frame)
        .on_start(pacman_start)
        .install();
}