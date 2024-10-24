use super::*;

//---------------------BLACKHOLE-----------------------

// PRE
unsafe extern "C" fn blackhole_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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

// INIT
unsafe extern "C" fn blackhole_init(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 15.0, z: 30.0});

    return 0.into();
}

pub fn install() {
    Agent::new("palutena_blackhole")
        .status(Pre, *WEAPON_PALUTENA_BLACKHOLE_STATUS_KIND_BLACKHOLE, blackhole_pre)
        .status(Init, *WEAPON_PALUTENA_BLACKHOLE_STATUS_KIND_BLACKHOLE, blackhole_init)

        .install();
}