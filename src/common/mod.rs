use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*
};

use skyline::hooks::{getRegionAddress, Region};

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

pub fn install() {
    skyline::install_hooks!(
        is_valid_just_shield_reflector,
        normal_weapon_hit_handler
    );
}
