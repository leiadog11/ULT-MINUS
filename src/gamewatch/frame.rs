use super::*;

// OPFF
pub unsafe extern "C" fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma);
        let lr = PostureModule::lr(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let posx = PostureModule::pos_x(boma);

        // ON GAME OVER - RESET VARIABLES
        if status_kind == *FIGHTER_STATUS_KIND_DEMO { 
            BOMB_OUT[ENTRY_ID] = false;
        }

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            BOMB_OUT[ENTRY_ID] = false;
        }

        // ON DEATH
        if status_kind == *FIGHTER_STATUS_KIND_DEAD { 
            BOMB_OUT[ENTRY_ID] = false;
        }

        // MOVE ON DOWN TILT
        if motion_kind == smash::hash40("attack_lw3") {
            if frame >= 6.0 && frame <= 8.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::set_rate(boma, 0.0);
                    if lr == 1.0 {
                        PostureModule::set_pos_2d(boma, &Vector2f {x: posx + 1.1, y: PostureModule::pos_y(boma)});
                    } 
                    else {
                        PostureModule::set_pos_2d(boma, &Vector2f {x: posx - 1.1, y: PostureModule::pos_y(boma)});
                    }
                }
                else {
                    MotionModule::set_rate(boma, 1.0);
                }
            }
        }

        // SPINNING HITSTUN
        if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY].contains(&status_kind) { 
            DAMAGE_ROT[ENTRY_ID] += 14.0;
            ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{x: DAMAGE_ROT[ENTRY_ID], y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        } else {
            DAMAGE_ROT[ENTRY_ID] = 0.0;
        }
    }
}

// ON START
pub unsafe extern "C" fn gamewatch_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        OCTOPUS[ENTRY_ID] = false;
        BOMB_OUT[ENTRY_ID] = false;
        DAMAGE_ROT[ENTRY_ID] = 0.0;
    }
}

pub fn install() {
    Agent::new("gamewatch")
        .on_line(Main, gamewatch_frame)
        .on_start(gamewatch_start)
        .install();
}