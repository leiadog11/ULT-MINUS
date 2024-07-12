use super::*;

// OPFF
pub unsafe extern "C" fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if DamageModule::damage(fighter.module_accessor, 0) >= 150.0 {
            if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_BLOWN_UP) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("self_destruct"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_BLOWN_UP);
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL || status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

// ON START
pub unsafe extern "C" fn pichu_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_BLOWN_UP);
    }
}

pub fn install() {
    Agent::new("pichu")
        .on_line(Main, pichu_frame)
        .on_start(pichu_start)
        .install();
}