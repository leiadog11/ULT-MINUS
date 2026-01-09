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

// UP TILT
unsafe extern "C" fn peach_attack_hi3_check_attack_status(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let rand = smash::app::sv_math::rand(hash40("fighter"), 20) as u64;
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if SLEEP_MOVE[ENTRY_ID] {
                ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_FINALPEACH), 0, 0, false, false);
                SLEEP_MOVE[ENTRY_ID] = false;
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("peach")
        .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_HI3, peach_attack_hi3_check_attack_status)

        .install();
}