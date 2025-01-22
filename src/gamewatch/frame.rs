use super::*;

// OPFF
pub unsafe extern "C" fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);

        // MOVE ON DOWN TILT
        if motion_kind == smash::hash40("attack_lw3") {
            if frame >= 6.0 && frame <= 8.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::set_rate(fighter.module_accessor, 0.0);
                    if lr == 1.0 {
                        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 1.1, y: PostureModule::pos_y(fighter.module_accessor)});
                    } 
                    else {
                        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 1.1, y: PostureModule::pos_y(fighter.module_accessor)});
                    }
                }
                else {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                }
            }
        }

        // MOVE ON DOWN B IN THE AIR
        if motion_kind == smash::hash40("special_lw") {
            if situation_kind == *SITUATION_KIND_AIR {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 1.1, y: PostureModule::pos_y(fighter.module_accessor)});
            }
        }

        // INVISIBLE FIX
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            VisibilityModule::set_whole(fighter.module_accessor, true);
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("blockm"), false);
        }
    }
}

// ON START
pub unsafe extern "C" fn gamewatch_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_OCTOPUS);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT);
    }
}

pub fn install() {
    Agent::new("gamewatch")
        .on_line(Main, gamewatch_frame)
        .on_start(gamewatch_start)
        .install();
}