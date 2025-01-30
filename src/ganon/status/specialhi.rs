use super::*;

///////////// SPECIAL HI

// INIT
unsafe extern "C" fn ganon_specialhi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if GROUND_CHECK[get_entry_id(fighter.module_accessor)] {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

pub fn install() {
    Agent::new("ganon")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_specialhi_init)
        .install();
}