use super::*;

// FORWARD SMASH START 

// MAIN
unsafe extern "C" fn ganon_attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackS4Start();
    }
    fighter.status_AttackS4Start_Common();
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attack_s4_start_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attack_s4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Start_Main();
}

// FORWARD SMASH HOLD

// MAIN
unsafe extern "C" fn ganon_attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackS4Hold();
    }
    fighter.status_AttackS4Hold();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_hold2"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attack_s4_hold_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attack_s4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Hold_main();
}

// FOWARD SMASH

// MAIN
unsafe extern "C" fn ganon_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackS4();
    }
    fighter.status_AttackS4();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attack_s4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw4_Main();
}

pub fn install() {
    Agent::new("ganon")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_START, ganon_attack_s4_start_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, ganon_attack_s4_hold_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, ganon_attack_s4_main)
    .install();
}