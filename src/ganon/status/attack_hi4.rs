use super::*;

// UP SMASH START 

// MAIN
unsafe extern "C" fn ganon_attackhi4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackHi4Start();
    }
    fighter.status_AttackHi4Start();
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi42"), 0.0, 1.0, false, 0.0, false, false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attackhi4_start_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attackhi4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackHi4Start_Main();
}

// UP SMASH HOLD

// MAIN
unsafe extern "C" fn ganon_attackhi4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackHi4Hold();
    }
    fighter.status_AttackHi4Hold();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi4_hold2"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attackhi4_hold_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attackhi4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackHi4Hold_main();
}

// UP SMASH

// MAIN
unsafe extern "C" fn ganon_attackhi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackHi4();
    }
    fighter.status_AttackHi4();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi42"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attackhi4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attackhi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackHi4_Main();
}

pub fn install() {
    Agent::new("ganon")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, ganon_attackhi4_start_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, ganon_attackhi4_hold_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, ganon_attackhi4_main)
    .install();
}