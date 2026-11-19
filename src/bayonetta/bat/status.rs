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

    weapon.fastshift(L2CValue::Ptr(bat_specialhi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bat_specialhi_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let owner_x = PostureModule::pos_x(owner_boma);
    let owner_y = PostureModule::pos_y(owner_boma);
    let owner_z = PostureModule::pos_z(owner_boma);

    let new_pos = Vector3f{x: owner_x, y: owner_y, z: owner_z};
    PostureModule::set_pos(weapon.module_accessor, &new_pos);

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