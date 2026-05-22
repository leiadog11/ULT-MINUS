use super::*;

// OPFF
pub unsafe extern "C" fn gekkouga_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let pos_x = PostureModule::pos_x(boma);
        let pos_y = PostureModule::pos_y(boma);
        let lr = PostureModule::lr(boma);
        let status_kind = StatusModule::status_kind(boma);

        // JUMP CANCEL DOWN AIR
        if MotionModule::motion_kind(boma) == hash40("attack_air_lw") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn gekkouga_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("gekkouga")
        .on_line(Main, gekkouga_frame)
        .on_start(gekkouga_start)
        .install();
}