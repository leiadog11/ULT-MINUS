use super::*;

//---------------------SHOT-----------------------

// PRE
unsafe extern "C" fn autoaimbullet_shot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *WEAPON_KINETIC_TYPE_NONE, 
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
unsafe extern "C" fn autoaimbullet_shot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shot"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);
    //Snap to throw position
    let mut offset_add = Vector3f{x:0.0,y:4.0,z:0.0};
    let mut newPos = Vector3f{x:0.0,y:0.0,z:0.0};

    let lr = PostureModule::lr(owner_boma);
    if lr == 1.0 {
        newPos = Vector3f{x: PostureModule::pos_x(owner_boma) + offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};
    } 
    else {
        newPos = Vector3f{x: PostureModule::pos_x(owner_boma) - offset_add.x, y: PostureModule::pos_y(owner_boma) + offset_add.y, z: PostureModule::pos_z(owner_boma)};
    }

    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 90.0}, 0);
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    BULLET_X_POS[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
    BULLET_Y_POS[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);

    weapon.fastshift(L2CValue::Ptr(autoaimbullet_shot_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn autoaimbullet_shot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ypos = ControlModule::get_stick_y(owner_boma);

    if MotionModule::motion_kind(owner_boma) == hash40("special_lw") {
        if MotionModule::frame(owner_boma) >= 2.0 && MotionModule::frame(owner_boma) <= 3.0  {
            autoaimbullet_remove(weapon);
            return 0.into();
        }
    }

    if MotionModule::motion_kind(owner_boma) == hash40("attack_hi4") {
        if MotionModule::frame(owner_boma) >= 15.0 {
            autoaimbullet_remove(weapon);
            return 0.into();
        }
    }
    if ANCHOR_PLANTED[get_entry_id(owner_boma)] {
        if StatusModule::status_kind(owner_boma) == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 && StatusModule::situation_kind(owner_boma) == *SITUATION_KIND_AIR { 
            autoaimbullet_remove(weapon);
            return 0.into();
        }
    }

    return 0.into();
}

// END
unsafe extern "C" fn autoaimbullet_shot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn autoaimbullet_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("palutena_autoaimbullet")
        .status(Pre, *WEAPON_PALUTENA_AUTOAIMBULLET_STATUS_KIND_SHOT, autoaimbullet_shot_pre)
        .status(Main, *WEAPON_PALUTENA_AUTOAIMBULLET_STATUS_KIND_SHOT, autoaimbullet_shot_main)
        .status(End, *WEAPON_PALUTENA_AUTOAIMBULLET_STATUS_KIND_SHOT, autoaimbullet_shot_end)

        .install();
}