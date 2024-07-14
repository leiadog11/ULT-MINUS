use super::*;

//------------CHECK ATTACK-------------

unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}


// DOWN TILT
unsafe extern "C" fn luigi_attack_lw3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            WorkModule::add_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }
    }
    0.into()
}

// NAIR
unsafe extern "C" fn luigi_attack_airn_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
	        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
		        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
	        }
	    }
    }
    0.into()
}

// UP SMASH
unsafe extern "C" fn luigi_attack_hi4_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4) {
                GroundModule::set_collidable(opponent_boma, false);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_ATTACK_HI4);
            }
	    }
    }
    0.into()
}

// UP B
unsafe extern "C" fn luigi_special_hi_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
	        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT) {
		        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_FALL.into(), false.into());
     	    }
	    }
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi")
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW3, luigi_attack_lw3_check_attack_status)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, luigi_attack_airn_check_attack_status)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_HI4, luigi_attack_hi4_check_attack_status)
        .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_HI, luigi_special_hi_check_attack_status)
        .install();
}