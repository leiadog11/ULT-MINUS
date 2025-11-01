use super::*;

// OPFF
pub unsafe extern "C" fn roy_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let frame = MotionModule::frame(boma);
        let lr = PostureModule::lr(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let posx = PostureModule::pos_x(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
            remove_pyra(boma);
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 { // REMOVE SWORD
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            remove_pyra(boma);
        }

        // REMOVE PYRA
        if (motion_kind != hash40("appeal_hi_r") && motion_kind != hash40("appeal_hi_l")) && !PYRA_REMOVED[ENTRY_ID] {
            remove_pyra(boma);
        }
    }
}

// ON START
pub unsafe extern "C" fn roy_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        PYRA_REMOVED[ENTRY_ID] = true;
        remove_pyra(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("roy")
        .on_line(Main, roy_frame)
        .on_start(roy_start)
        .install();
}