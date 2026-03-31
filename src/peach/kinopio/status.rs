use super::*;

// -------- APPEAR --------

// PRE
unsafe extern "C" fn kinopio_appear_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_GROUND as u32, 
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
unsafe extern "C" fn kinopio_appear_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("appear"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(kinopio_appear_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_appear_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::is_end(weapon.module_accessor) { 
        weapon.change_status(WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_appear_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// -------- WAIT --------

// PRE
unsafe extern "C" fn kinopio_wait_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_GROUND as u32, 
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
unsafe extern "C" fn kinopio_wait_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);

    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }

    weapon.fastshift(L2CValue::Ptr(kinopio_wait_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_wait_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let xpos = ControlModule::get_stick_x(owner_boma);
    let ypos = ControlModule::get_stick_y(owner_boma);

    // PEACH PRESSES A - PUNCH
    if ControlModule::check_button_on(owner_boma, *CONTROL_PAD_BUTTON_ATTACK) { 
        weapon.change_status(WEAPON_PEACH_KINOPIO_STATUS_KIND_PUNCH.into(), false.into());
        return 1.into();
    }

    // PEACH PRESSES B - SPRAY
    if ControlModule::check_button_on(owner_boma, *CONTROL_PAD_BUTTON_SPECIAL) && xpos == 0.0 && ypos == 0.0 {
        weapon.change_status(WEAPON_PEACH_KINOPIO_STATUS_KIND_SPRAY.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_wait_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// -------- PUNCH --------

// PRE
unsafe extern "C" fn kinopio_punch_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_GROUND as u32, 
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
unsafe extern "C" fn kinopio_punch_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("punch"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(kinopio_punch_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_punch_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::is_end(weapon.module_accessor) { 
        weapon.change_status(WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_punch_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// -------- SPRAY --------

// PRE
unsafe extern "C" fn kinopio_spray_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_GROUND as u32, 
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
unsafe extern "C" fn kinopio_spray_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("spray"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(kinopio_spray_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_spray_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::is_end(weapon.module_accessor) { 
        kinopio_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_spray_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// -------- AIR SPRAY --------

// PRE
unsafe extern "C" fn kinopio_air_spray_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_AIR as u32, 
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
unsafe extern "C" fn kinopio_air_spray_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("air_spray"), 0.0, 1.0, false, 0.0, false, false);

    // LINK TO PEACH
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 8.0, z: 0.0});

    weapon.fastshift(L2CValue::Ptr(kinopio_air_spray_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_air_spray_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::is_end(weapon.module_accessor) { 
        kinopio_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_air_spray_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// -------- FALL --------

// PRE
unsafe extern "C" fn kinopio_fall_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_AIR as u32, 
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
unsafe extern "C" fn kinopio_fall_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(kinopio_fall_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_fall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 

    // FALL DOWN

    // LAND
    if StatusModule::situation_kind(weapon.module_accessor) == *SITUATION_KIND_GROUND { 
        weapon.change_status(WEAPON_PEACH_KINOPIO_STATUS_KIND_LAND.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_fall_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// -------- LAND --------

// PRE
unsafe extern "C" fn kinopio_land_pre(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        *GROUND_CORRECT_KIND_GROUND as u32, 
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
unsafe extern "C" fn kinopio_land_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("land"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(kinopio_land_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn kinopio_land_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::is_end(weapon.module_accessor) { 
        weapon.change_status(WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn kinopio_land_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn kinopio_remove(weapon: &mut L2CWeaponCommon) {
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("peach_kinopio")
        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_APPEAR, kinopio_appear_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_APPEAR, kinopio_appear_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_APPEAR, kinopio_appear_end)

        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT, kinopio_wait_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT, kinopio_wait_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_WAIT, kinopio_wait_end)

        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_PUNCH, kinopio_punch_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_PUNCH, kinopio_punch_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_PUNCH, kinopio_punch_end)

        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_SPRAY, kinopio_spray_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_SPRAY, kinopio_spray_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_SPRAY, kinopio_spray_end)

        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_AIR_SPRAY, kinopio_air_spray_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_AIR_SPRAY, kinopio_air_spray_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_AIR_SPRAY, kinopio_air_spray_end)

        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, kinopio_fall_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, kinopio_fall_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, kinopio_fall_end)

        .status(Pre, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, kinopio_land_pre)
        .status(Main, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, kinopio_land_main)
        .status(End, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, kinopio_land_end)

        .install();
}