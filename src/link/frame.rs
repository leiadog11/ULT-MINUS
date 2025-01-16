use super::*;

// OPFF
pub unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if situation_kind == *SITUATION_KIND_AIR {
                if frame == 1.0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, true);
                }
            }
        }

        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_USED);
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_USED) &&
        !WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED) {
            if situation_kind == *SITUATION_KIND_AIR { 
                if stick_y > 0.5 && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, true);
                }
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn link_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_USED);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED);
    }
}

pub fn install() {
    Agent::new("link")
        .on_line(Main, link_frame)
        .on_start(link_start)
        .install();
}