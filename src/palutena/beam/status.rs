use super::*;

//---------------------BEAM-----------------------

// PRE
unsafe extern "C" fn beam_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
unsafe extern "C" fn beam_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("beam"), 0.0, 1.0, false, 0.0, false, false);
    println!("BEAM STATUS!");
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Snap to throw position
    let mut offset_add = Vector3f{x:10.0,y:50.0,z:0.0};
    let mut newPos = Vector3f{x:0.0,y:0.0,z:0.0};

    let lr = PostureModule::lr(owner_boma);
    if lr == 1.0 {
        newPos = Vector3f{x: PostureModule::pos_x(owner_boma) + offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};
    } 
    else {
        newPos = Vector3f{x: PostureModule::pos_x(owner_boma) - offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};
    }

    PostureModule::set_pos(weapon.module_accessor, &newPos);

    println!("Beam X: {}", PostureModule::pos_x(weapon.module_accessor));
    println!("Beam Y: {}", PostureModule::pos_y(weapon.module_accessor));

    weapon.fastshift(L2CValue::Ptr(beam_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn beam_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// END
unsafe extern "C" fn beam_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("palutena_beam")
        .status(Pre, *WEAPON_PALUTENA_BEAM_STATUS_KIND_BEAM, beam_pre)
        .status(Main, *WEAPON_PALUTENA_BEAM_STATUS_KIND_BEAM, beam_main)
        .status(End, *WEAPON_PALUTENA_BEAM_STATUS_KIND_BEAM, beam_end)

        .install();
}