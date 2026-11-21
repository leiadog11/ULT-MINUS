use super::*;

// INIT
unsafe extern "C" fn bayonetta_special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_N_FOOT);
    return 0.into();
}

pub fn install() {
    Agent::new("bayonetta")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, bayonetta_special_n_init)
    .install();
}