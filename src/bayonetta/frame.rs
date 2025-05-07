use super::*;

// OPFF
pub unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let currentsize = PostureModule::scale(boma);
        let entry_id = WorkModule::get_int(boma,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::set_rate(fighter.module_accessor, 0.3);            
            }
            else {
                MotionModule::set_rate(fighter.module_accessor, 3.2);
            }
        }
        if DamageModule::damage(boma, 0) >= 50.0 {
            smash::app::lua_bind::FighterManager::set_final(fighter_manager,FighterEntryID(entry_id),smash::app::FighterAvailableFinal(*(smash::lib::lua_const::FighterAvailableFinal::DEFAULT)),0u32);        }
        
        //BAYO GROWS LARGER IF NAIR IS HELD
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n_hold") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                PostureModule::set_scale(boma, currentsize*1.04, false);
            }
        }

        //SET BAYO SIZE BACK TO NORMAL AFTER NAIR IS COMPLETE
        if MotionModule::motion_kind(boma) == hash40("attack_air_n_hold") {
            if MotionModule::is_end(boma) {
                PostureModule::set_scale(boma, 1.00, false);
            }
        }

        //SET BAYO SIZE BACK TO NORMAL AFTER GETTING HIT DURING NAIR
        if MotionModule::motion_kind(boma) == hash40("attack_air_n_hold") ||
        DamageModule::reaction(boma, 0) > 1.0 {
            PostureModule::set_scale(boma, 1.00, false);
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