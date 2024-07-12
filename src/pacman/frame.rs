use super::*;

// OPFF
pub unsafe extern "C" fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
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

        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD {
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("ghost1"), &Vector3f{x: 0.0, y: -90.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("ghost2"), &Vector3f{x: 0.0, y: -90.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
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