use super::*;

/////////// BEAM FLY

/*
// INIT
unsafe extern "C" fn rob_beam_fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    GroundModule::set_test_coll_stop_status(weapon.module_accessor, true);
    return 0.into();
}

// PRE
unsafe extern "C" fn rob_beam_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn rob_beam_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_ROBOT_BEAM_INSTANCE_WORK_ID_FLAG_STRONG) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly_max"), 0.0, 1.0, false, 0.0, false, false);
    } 

    if StopModule::is_stop(weapon.module_accessor) {
    //FUN HAHAHA
    }
  
    weapon.fastshift(L2CValue::Ptr(rob_beam_fly_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn rob_beam_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn rob_beam_fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// EXEC
unsafe extern "C" fn rob_beam_fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //FUN HAHAHA
    return 0.into();
}

// EXEC STOP
unsafe extern "C" fn rob_beam_fly_exec_stop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //FUN HAHAHA
    return 0.into();
}
*/

pub fn install() {
    Agent::new("robot_beam")
        /*
        .status(Init, *WEAPON_ROBOT_BEAM_STATUS_KIND_FLY, rob_beam_fly_init)
        .status(Exec, *WEAPON_ROBOT_BEAM_STATUS_KIND_FLY, rob_beam_fly_exec)
        .status(Pre, *WEAPON_ROBOT_BEAM_STATUS_KIND_FLY, rob_beam_fly_pre)
        .status(Main, *WEAPON_ROBOT_BEAM_STATUS_KIND_FLY, rob_beam_fly_main)
        .status(End, *WEAPON_ROBOT_BEAM_STATUS_KIND_FLY, rob_beam_fly_end)
        */
        .install();
}