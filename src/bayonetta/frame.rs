use super::*;

// OPFF
pub unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::set_rate(fighter.module_accessor, 0.2);            
            }
            else {
                MotionModule::set_rate(fighter.module_accessor, 3.5);
            }
        }
        if DamageModule::damage(boma, 0) >= 50.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
        }
    }
}

// ON START
pub unsafe extern "C" fn bayonetta_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("bayonetta")
        .on_line(Main, bayonetta_frame)
        .on_start(bayonetta_start)
        .install();
}