use super::*;

// DOWN SMASH START 

// MAIN
unsafe extern "C" fn ganon_attack_lw4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackLw4Start();
    }
    fighter.status_AttackLw4Start();
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw42"), 0.0, 1.0, false, 0.0, false, false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attack_lw4_start_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attack_lw4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw4Start_Main();
}

// DOWN SMASH HOLD

// MAIN
unsafe extern "C" fn ganon_attack_lw4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackLw4Hold();
    }
    fighter.status_AttackLw4Hold();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold2"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attack_lw4_hold_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attack_lw4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw4Hold_main();
}

// DOWN SMASH

// MAIN
unsafe extern "C" fn ganon_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if SWORD[get_entry_id(fighter.module_accessor)] {
        return fighter.status_AttackLw4();
    }
    fighter.status_AttackLw4();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw42"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_attack_lw4_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw4_Main();
}

pub fn install() {
    Agent::new("ganon")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, ganon_attack_lw4_start_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, ganon_attack_lw4_hold_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, ganon_attack_lw4_main)
    .install();
}