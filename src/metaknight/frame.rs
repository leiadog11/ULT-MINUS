use super::*;

// OPFF
pub unsafe extern "C" fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma);

        //TORNADO FRAME 1 INVINCIBLE
        if status_kind == FIGHTER_STATUS_KIND_SPECIAL_N {
            if frame == 1.0 {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            }
            else {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn metaknight_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("metaknight")
        .on_line(Main, metaknight_frame)
        .on_start(metaknight_start)
        .install();
}