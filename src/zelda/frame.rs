use super::*;

// OPFF
pub unsafe extern "C" fn zelda_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let status_kind = StatusModule::status_kind(boma);

        // //ACT OUT OF UP SPECIAL
        // if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        //     if frame > 38.0 {
            // StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        //     }
        // }
    }
}


// ON START
pub unsafe extern "C" fn zelda_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("zelda")
        .on_line(Main, zelda_frame)
        .on_start(zelda_start)
        .install();
}