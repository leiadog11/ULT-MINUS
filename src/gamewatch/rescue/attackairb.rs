use super::*;

/////////// RESCUE BACK AIR

// PRE
unsafe extern "C" fn gamewatch_rescue_attackairb_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
unsafe extern "C" fn gamewatch_rescue_attackairb_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("attack_air_b"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(gamewatch_rescue_attackairb_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn gamewatch_rescue_attackairb_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_posx = PostureModule::pos_x(owner_boma);
    let owner_posy = PostureModule::pos_y(owner_boma);
    let owner_posz = PostureModule::pos_z(owner_boma);
    let lr = PostureModule::lr(owner_boma);
    let frame = MotionModule::frame(owner_boma);
    if frame >= 1.0 && frame <= 13.0 {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
        if lr == 1.0 {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx + 4.0, y: owner_posy + 2.0, z: owner_posz});
        }
        else {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx - 4.0, y: owner_posy + 2.0, z: owner_posz - 2.0});
        }
        return 0.into();
    }
    if frame >= 14.0 && frame <= 27.0 {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
        if lr == 1.0 {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx + 4.0, y: owner_posy + 3.0, z: owner_posz});
        }
        else {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx - 4.0, y: owner_posy + 3.0, z: owner_posz - 2.0});
        }
        return 0.into();
    }
    if frame >= 28.0 && frame <= 42.0 {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
        if lr == 1.0 {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx + 4.0, y: owner_posy + 4.0, z: owner_posz});
        }
        else {
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx - 4.0, y: owner_posy + 4.0, z: owner_posz - 2.0});
        }
        return 0.into();
    }

    if MotionModule::is_end(owner_boma) {
        return 0.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn gamewatch_rescue_attackairb_end(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("gamewatch_rescue")
        .status(Pre, WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACKAIRB, gamewatch_rescue_attackairb_pre)
        .status(Main, WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACKAIRB, gamewatch_rescue_attackairb_main)
        .status(End, WEAPON_GAMEWATCH_RESCUE_STATUS_KIND_ATTACKAIRB, gamewatch_rescue_attackairb_end)
        .install();
}