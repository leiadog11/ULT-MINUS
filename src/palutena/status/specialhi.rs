use super::*;

//--------------------------SPECIALHI---------------------------

// PRE
unsafe extern "C" fn palutena_specialhi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if UP_B_USED[get_entry_id(fighter.module_accessor)] {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }

    return 0.into();
}

pub fn install() {
    Agent::new("palutena")
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, palutena_specialhi_exec)
        .install();
}