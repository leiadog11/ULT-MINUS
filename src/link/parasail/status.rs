use super::*;

//-------------------SPECIAL AIR HI START------------------------

// PRE
unsafe extern "C" fn parasail_specialairhistart_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_RESET, 
        *GROUND_CORRECT_KIND_NONE as u32, 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn parasail_specialairhistart_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(parasail_specialairhistart_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn parasail_specialairhistart_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn parasail_specialairhistart_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

//-------------------SPECIAL AIR HI GLIDE------------------------

// PRE
unsafe extern "C" fn parasail_specialairhiglide_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_RESET, 
        *GROUND_CORRECT_KIND_NONE as u32, 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn parasail_specialairhiglide_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_hi_glide"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(parasail_specialairhiglide_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn parasail_specialairhiglide_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn parasail_specialairhiglide_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

//-------------------SPECIAL AIR HI EQUIP/UNEQUIP------------------------

// PRE
unsafe extern "C" fn parasail_specialairhiequip_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_RESET, 
        *GROUND_CORRECT_KIND_NONE as u32, 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn parasail_specialairhiequip_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    if EQUIPPED[get_entry_id(owner_boma)] { 
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_hi_unequip"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_hi_equip"), 0.0, 1.0, false, 0.0, false, false);
    }

    weapon.fastshift(L2CValue::Ptr(parasail_specialairhiequip_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn parasail_specialairhiequip_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn parasail_specialairhiequip_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("link_parasail")
    .status(Pre, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START, parasail_specialairhistart_pre)
    .status(Main, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START, parasail_specialairhistart_main)
    .status(End, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START, parasail_specialairhistart_end)

    .status(Pre, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, parasail_specialairhiglide_pre)
    .status(Main, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, parasail_specialairhiglide_main)
    .status(End, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, parasail_specialairhiglide_end)

    .status(Pre, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, parasail_specialairhiequip_pre)
    .status(Main, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, parasail_specialairhiequip_main)
    .status(End, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, parasail_specialairhiequip_end)

    .install();

}