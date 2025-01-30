use super::*;

//------------------SPECIAL LW--------------------

// INIT
unsafe extern "C" fn captain_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !KICK[get_entry_id(fighter.module_accessor)] {
        fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

pub fn install() {
    Agent::new("captain")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, captain_speciallw_init)

        .install();
}

