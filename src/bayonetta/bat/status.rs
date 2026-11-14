use super::*;

// -------- SPECIAL HI --------

// PRE
unsafe extern "C" fn bat_specialhi_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    
    return 0.into();
}

// MAIN
unsafe extern "C" fn bat_specialhi_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    weapon.fastshift(L2CValue::Ptr(bat_specialhi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bat_specialhi_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn bat_remove(weapon: &mut L2CWeaponCommon) {
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("bayonetta_bat")
        .status(Pre, WEAPON_BAYONETTA_BAT_STATUS_KIND_SPECIAL_HI, bat_specialhi_pre)
        .status(Main, WEAPON_BAYONETTA_BAT_STATUS_KIND_SPECIAL_HI, bat_specialhi_main)
    .install();
}