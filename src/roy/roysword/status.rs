use super::*;

/////// REGULAR

// PRE
unsafe extern "C" fn roy_roysword_regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_AIR.into(), 
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
unsafe extern "C" fn roy_roysword_regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let life = 132;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("top"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_translate_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 12.0, z: 12.0});
    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }

    weapon.fastshift(L2CValue::Ptr(roy_roysword_regular_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn roy_roysword_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(weapon.module_accessor);

    let pos_x = PostureModule::pos_x(owner_boma) + 12.0;
    let pos_y = PostureModule::pos_y(owner_boma) + 12.0;
    let pos_z = PostureModule::pos_z(owner_boma);

    let mut newPos = Vector3f{x: pos_x + 2.0, y: pos_y, z: pos_z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    // REFLECTION CHECK
    if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.5, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        return 0.into();
    }

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        roysword_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn roysword_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("roy_roysword")
        .status(Pre, WEAPON_ROY_ROYSWORD_STATUS_KIND_REGULAR, roy_roysword_regular_pre)
        .status(Main, WEAPON_ROY_ROYSWORD_STATUS_KIND_REGULAR, roy_roysword_regular_main)

        .install();
}