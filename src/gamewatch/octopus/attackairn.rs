use super::*;

/////////// OCTOPUS NEUTRAL AIR 

// PRE
unsafe extern "C" fn gamewatch_octopus_attackairn_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *WEAPON_KINETIC_TYPE_NONE, 
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn gamewatch_octopus_attackairn_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_air_n"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(gamewatch_octopus_attackairn_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn gamewatch_octopus_attackairn_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::frame(weapon.module_accessor) >= 15.0{
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        
        return 0.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn gamewatch_octopus_attackairn_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}




pub fn install() {
    Agent::new("gamewatch_octopus")
        .status(Pre, WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN, gamewatch_octopus_attackairn_pre)
        .status(Main, WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN, gamewatch_octopus_attackairn_main)
        .status(End, WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN, gamewatch_octopus_attackairn_end)
        .install();
}