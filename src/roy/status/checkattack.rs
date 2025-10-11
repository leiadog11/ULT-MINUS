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

// NAIR
unsafe extern "C" fn roy_attack_airn_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
	        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
		        if MotionModule::frame(fighter.module_accessor) >= 14.0{
                    macros::SET_SPEED_EX(fighter, 0.0, 3.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                }
	        }
	    }
    }
    0.into()
}

// DOWN SMASH
unsafe extern "C" fn roy_attack_lw4_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
	        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw4") {
                cancel_with_dash(fighter.module_accessor, 7.0);
	        }
	    }
    }
    0.into()
}

pub fn install() {
    Agent::new("roy")
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, roy_attack_airn_check_attack_status)

        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW4, roy_attack_lw4_check_attack_status)

        .install();
}