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

// FORWARD TILT
unsafe extern "C" fn jack_attack_s3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
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

// // FORWARD TILT CURSE_TIMER
// unsafe extern "C" fn jack_attack_s3_curse_timer_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
//     let table = param_3.get_table() as *mut smash2::lib::L2CTable;
//     let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
//     let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
//     let rand = smash::app::sv_math::rand(hash40("fighter"), 5) as u64;
//     let ENTRY_ID = get_entry_id(fighter.module_accessor);
//     if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
//         if collision_kind == *COLLISION_KIND_HIT {
//             let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
//             let opponent_boma = sv_battle_object::module_accessor(object_id);
//     if CURSE_TIMER[ENTRY_ID] > 0 {
//         if rand == 1 {
//             StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_BIND, false);
//         }
//     }
//     else {
//         CURSE_TIMER[ENTRY_ID] = 360;
//     }
//         }
//     }
//     0.into()
// }

// // DOWN TILT CURSE_TIMER
// unsafe extern "C" fn jack_attack_lw3_curse_timer_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
//     let table = param_3.get_table() as *mut smash2::lib::L2CTable;
//     let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
//     let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
//     let rand = smash::app::sv_math::rand(hash40("fighter"), 5) as u64;
//     let ENTRY_ID = get_entry_id(fighter.module_accessor);
//     if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
//         if collision_kind == *COLLISION_KIND_HIT {
//             let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
//             let opponent_boma = sv_battle_object::module_accessor(object_id);
//     if CURSE_TIMER[ENTRY_ID] > 0 {
//         if rand == 1 {
//             StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_BIND, false);
//         }
//     }
//     else {
//         CURSE_TIMER[ENTRY_ID] = 360;
//     }
//         }
//     }
//     0.into()
// }

// // DASH ATTACK CURSE_TIMER
// unsafe extern "C" fn jack_attack_dash_curse_timer_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
//     let table = param_3.get_table() as *mut smash2::lib::L2CTable;
//     let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
//     let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
//     let rand = smash::app::sv_math::rand(hash40("fighter"), 5) as u64;
//     let ENTRY_ID = get_entry_id(fighter.module_accessor);
//     if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
//         if collision_kind == *COLLISION_KIND_HIT {
//             let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
//             let opponent_boma = sv_battle_object::module_accessor(object_id);
//     if CURSE_TIMER[ENTRY_ID] > 0 {
//         if rand == 1 {
//             StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_BIND, false);
//         }
//     }
//     else {
//         CURSE_TIMER[ENTRY_ID] = 360;
//     }
//         }
//     }
//     0.into()
// }

// // DOWN SMASH CURSE_TIMER
// unsafe extern "C" fn jack_attack_lw4_curse_timer_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
//     let table = param_3.get_table() as *mut smash2::lib::L2CTable;
//     let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
//     let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
//     let rand = smash::app::sv_math::rand(hash40("fighter"), 5) as u64;
//     let ENTRY_ID = get_entry_id(fighter.module_accessor);
//     if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
//         if collision_kind == *COLLISION_KIND_HIT {
//             let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
//             let opponent_boma = sv_battle_object::module_accessor(object_id);
//     if CURSE_TIMER[ENTRY_ID] > 0 {
//         if rand == 1 {
//             StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_BIND, false);
//         }
//     }
//     else {
//         CURSE_TIMER[ENTRY_ID] = 360;
//     }
//         }
//     }
//     0.into()
// }

// // DOWN AIR CURSE_TIMER
// unsafe extern "C" fn jack_attack_air_lw_curse_timer_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
//     let table = param_3.get_table() as *mut smash2::lib::L2CTable;
//     let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
//     let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
//     let rand = smash::app::sv_math::rand(hash40("fighter"), 5) as u64;
//     let ENTRY_ID = get_entry_id(fighter.module_accessor);
//     if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
//         if collision_kind == *COLLISION_KIND_HIT {
//             let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
//             let opponent_boma = sv_battle_object::module_accessor(object_id);
//     if CURSE_TIMER[ENTRY_ID] > 0 {
//         if rand == 1 {
//             StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_BIND, false);
//         }
//     }
//     else {
//         CURSE_TIMER[ENTRY_ID] = 360;
//     }
//         }
//     }
//     0.into()
// }



pub fn install() {
    Agent::new("jack")
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S3, jack_attack_s3_check_attack_status)

        // .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S3, jack_attack_s3_curse_timer_check_attack_status)

        // .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW3, jack_attack_lw3_curse_timer_check_attack_status)

        // .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_DASH, jack_attack_dash_curse_timer_check_attack_status)

        // .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW4, jack_attack_lw4_curse_timer_check_attack_status)

        // .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, jack_attack_air_lw_curse_timer_check_attack_status)

        .install();
}