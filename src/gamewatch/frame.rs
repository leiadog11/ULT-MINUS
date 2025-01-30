use super::*;

// OPFF
pub unsafe extern "C" fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let frame = MotionModule::frame(boma);
        let lr = PostureModule::lr(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let posx = PostureModule::pos_x(boma);

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

        // MOVE ON DOWN B IN THE AIR
        if motion_kind == smash::hash40("special_lw") {
            if situation_kind == *SITUATION_KIND_AIR {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx + 1.1, y: PostureModule::pos_y(boma)});
            }
        }

        // INVISIBLE FIX
        if DamageModule::reaction(boma, 0) > 1.0 {
            VisibilityModule::set_whole(boma, true);
            ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ModelModule::set_mesh_visibility(boma, Hash40::new("blockm"), false);
        }
    }
}

// ON START
pub unsafe extern "C" fn gamewatch_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        
    }
}

pub fn install() {
    Agent::new("gamewatch")
        .on_line(Main, gamewatch_frame)
        .on_start(gamewatch_start)
        .install();
}