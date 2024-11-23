use super::*;

//////////// SPECIAL S DRIVE

// MAIN
unsafe extern "C" fn wario_specialsdrive_main(fighter: &mut L2CFighterCommon) -> L2CValue {
  MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
  fighter.sub_shift_status_main(L2CValue::Ptr(wario_specialsdrive_main_loop as *const () as _))
}
  
// MAIN LOOP
unsafe extern "C" fn wario_specialsdrive_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || //on A and Jump, do nothing
      ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        return 0.into();
    }
  
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || //on B or Shield, get off bike
      ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_S_FLAG_RESERVE_JUMP);
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START.into(), false.into());
        return 1.into();
      }
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_S_FLAG_RESERVE_JUMP) { //taunt on bike
      if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || 
          ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) ||
          ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) ||
          ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL.into(), false.into());
            return 1.into(); 
        }
    }
    return 0.into();
}

pub fn install() {
    Agent::new("wario")
        .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, wario_specialsdrive_main)
        .install();
}