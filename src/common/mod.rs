use super::*;

use skyline::hooks::{getRegionAddress, Region};
use skyline::nn::ro::LookupSymbol;

pub static mut FIGHTER_MANAGER: usize = 0;

// PARRY REFLECTS
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    return true;
}

// HIT HANDLER HOOK
#[skyline::hook(offset = 0x33bdc30)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let status_kind = StatusModule::status_kind(boma);
    let owner_kind = utility::get_kind(&mut *owner_boma);

    // G&W FLAG (RESCUE)
    if (*weapon).battle_object.kind == *WEAPON_KIND_GAMEWATCH_RESCUE as u32 && owner_kind == *FIGHTER_KIND_GAMEWATCH {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(vtable, weapon, log)
}

// GIVE FINAL SMASH
#[skyline::hook(offset=0x8dc140)]
pub unsafe fn give_final_smash(boma: *mut BattleObjectModuleAccessor) {
    let ENTRY_ID = get_entry_id(boma);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    smash::app::lua_bind::FighterManager::set_final(
        fighter_manager, 
        FighterEntryID(ENTRY_ID.try_into().unwrap()), 
        smash::app::FighterAvailableFinal { _address: *(smash::lib::lua_const::FighterAvailableFinal::DEFAULT) as u8 },
        0u32
    );
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
    skyline::install_hooks!(
        give_final_smash,
        is_valid_just_shield_reflector,
        normal_weapon_hit_handler
    );
}
