use super::*;

//------------------SPECIAL S--------------------

// PRE
unsafe extern "C" fn doyle_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn doyle_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(doyle_specials_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn doyle_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn doyle_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("jack_doyle")
        .status(Pre, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, doyle_specials_pre)
        .status(Main, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, doyle_specials_main)
        .status(End, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, doyle_specials_end)

        .install();
}