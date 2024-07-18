use super::*;

// OPFF
pub unsafe extern "C" fn reflectionboard_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        if WorkModule::is_flag(fighter.module_accessor, WEAPON_PALUTENA_REFLECTIONBOARD_INSTANCE_WORK_ID_FLAG_BREAK) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
            WorkModule::off_flag(fighter.module_accessor, WEAPON_PALUTENA_REFLECTIONBOARD_INSTANCE_WORK_ID_FLAG_BREAK);
        }
    }
}

// ON START
pub unsafe extern "C" fn reflectionboard_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, WEAPON_PALUTENA_REFLECTIONBOARD_INSTANCE_WORK_ID_FLAG_BREAK);
    }
}

pub fn install() {
    Agent::new("palutena_reflectionboard")
        .on_line(Main, reflectionboard_frame)
        .on_start(reflectionboard_start)
        .install();
}