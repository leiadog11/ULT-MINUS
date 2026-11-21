use super::*;

//----------------- UP TILT -----------------

// PRE
unsafe extern "C" fn rescue_attack_hi3_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *WEAPON_KINETIC_TYPE_NONE, 
        GROUND_CORRECT_KIND_AIR.into(), 
        GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn rescue_attack_hi3_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_hi3"), 0.0, 1.0, false, 0.0, false, false);

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_CONSTRAINT,Hash40::new("havel"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32, true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    weapon.fastshift(L2CValue::Ptr(rescue_attack_hi3_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn rescue_attack_hi3_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    if MotionModule::is_end(weapon.module_accessor) {
        rescue_remove(weapon);
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn rescue_remove(weapon: &mut smashline::L2CWeaponCommon) {
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("gamewatch_rescue")
        .status(Pre, WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACK_HI3, rescue_attack_hi3_pre)
        .status(Main, WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACK_HI3, rescue_attack_hi3_main)
    .install();
}