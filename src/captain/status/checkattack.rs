use super::*;

//-------CHECK ATTACK--------
unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

//RAPID JAB
unsafe extern "C" fn captain_attack_100_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let rand = smash::app::sv_math::rand(hash40("fighter"), 20) as u64;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
            SlowModule::set_whole(fighter.module_accessor, 8, 20);
            macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        }
    }
    0.into()
}

//F TILT
unsafe extern "C" fn captain_attack_s3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let rand = smash::app::sv_math::rand(hash40("fighter"), 20) as u64;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    0.into()
}

// UP TILT
unsafe extern "C" fn captain_attack_hi3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let rand = smash::app::sv_math::rand(hash40("fighter"), 20) as u64;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if MotionModule::frame(fighter.module_accessor) > 50.0 {
                SlowModule::set_whole(fighter.module_accessor, 8, 20);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
            }
        }
    }
    0.into()
}

//FORWARD AIR
unsafe extern "C" fn captain_attack_airf_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let rand = smash::app::sv_math::rand(hash40("fighter"), 20) as u64;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if MotionModule::frame(fighter.module_accessor) < 17.0 && MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") {
                SlowModule::set_whole(fighter.module_accessor, 120, 20);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 28.5,/*xrot*/ 60.0);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_dragoon_attack"));
            }
        }
    }
    0.into()
}


pub fn install() {
    Agent::new("captain")
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S3, captain_attack_s3_check_attack_status)

        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_HI3, captain_attack_hi3_check_attack_status)

        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_100, captain_attack_100_check_attack_status)

        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, captain_attack_airf_check_attack_status)

        .install();
}