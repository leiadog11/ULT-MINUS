use super::*;

//-----------------------CHECK------------------------

// PRE
unsafe extern "C" fn explosiveflame_check_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn explosiveflame_check_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("check"), 0.0, 1.0, false, 0.0, false, false);
    weapon.change_status((*WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE).into(), false.into());
    return 1.into();
}

// END
unsafe extern "C" fn explosiveflame_check_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}


//----------------------EXPLODE------------------------

// PRE
unsafe extern "C" fn explosiveflame_explode_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    return 0.into();
}

// INIT
unsafe extern "C" fn explosiveflame_explode_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Snap to throw position
    let mut owner_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut article_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut offset_add = Vector3f{x:22.0,y:14.0,z:0.0};

    PostureModule::set_scale(weapon.module_accessor, 1.5, false);

    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("throw")}, &mut owner_pos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut article_pos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + owner_pos.x - article_pos.x + (offset_add.x*lr), y: PostureModule::pos_y(owner) + owner_pos.y - (article_pos.y)+ offset_add.y, z: PostureModule::pos_z(owner) + owner_pos.z - article_pos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    return 0.into();
}

// MAIN
unsafe extern "C" fn explosiveflame_explode_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(explosiveflame_explode_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn explosiveflame_explode_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn explosiveflame_explode_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}


pub fn install() {
    Agent::new("palutena_explosiveflame")
        .status(Pre, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_CHECK, explosiveflame_check_pre)
        .status(Main, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_CHECK, explosiveflame_check_main)
        .status(End, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_CHECK, explosiveflame_check_end)

        .status(Pre, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explosiveflame_explode_pre)
        .status(Init, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explosiveflame_explode_init)
        .status(Main, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explosiveflame_explode_main)
        .status(End, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explosiveflame_explode_end)
        .install();
}