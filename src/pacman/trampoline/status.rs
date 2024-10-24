use super::*;

////////////// TRAMPOLINE START

// PRE
unsafe extern "C" fn pacman_trampoline_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
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
unsafe extern "C" fn pacman_trampoline_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(0x327ed09d4f));
    sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
    weapon.pop_lua_stack(1);

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_DEAD_AREA) {
        let trampoline_life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_trampoline"), hash40("life"));
        WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_INT_RIDER_NUM); 
    }
    else {
        WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_INT_RIDER_NUM); 
    }
    WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_AREA_BODY);
    let area_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_trampoline"), hash40("area_frame"));
    WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_INT_AREA_BODY_FRAME);
    weapon.fastshift(L2CValue::Ptr(pacman_trampoline_start_main_loop as *const () as _ ))
}

// MAIN LOOP
unsafe extern "C" fn pacman_trampoline_start_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.change_status((*WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_SHAKE).into(), false.into());
    return 1.into();
}

pub fn install() {
    Agent::new("pacman_trampoline")
        .status(Pre, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_START, pacman_trampoline_start_pre)
        .status(Main, *WEAPON_PACMAN_TRAMPOLINE_STATUS_KIND_START, pacman_trampoline_start_main)
        .install();
}