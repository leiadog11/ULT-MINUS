use super::*;

// OPFF
pub unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        /*
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, true);
            }
        }
        */
    }
}

// ON START
pub unsafe extern "C" fn link_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("link")
        .on_line(Main, link_frame)
        .on_start(link_start)
        .install();
}