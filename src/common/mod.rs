use super::*;

use skyline::hooks::{getRegionAddress, Region};
use skyline::nn::ro::LookupSymbol;

pub static mut FIGHTER_MANAGER: usize = 0;
static mut STALL_TIMER: [i32; 8] = [0; 8];

// PARRY REFLECTS
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    return true;
}

// HIT HANDLER HOOK
#[skyline::hook(offset = 0x33bd9c0)]
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

    // PIT SHIELD (CHARIOT)
    if (*weapon).battle_object.kind == *WEAPON_KIND_PIT_CHARIOT as u32 && owner_kind == *FIGHTER_KIND_PIT {
        *(weapon as *mut bool).add(0x90) = true;
    }

    call_original!(vtable, weapon, log)
}

// GLOBAl FIGHTER FRAME
pub unsafe extern "C" fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
        }

        // DANGER
        if situation_kind == *SITUATION_KIND_AIR {
            if STALL_TIMER[ENTRY_ID] == 720 {
                let vector = Vector3f{x:0.0,y:10.0,z:0.0};
                EffectModule::req_follow(boma, Hash40::new("sys_flies_up"), Hash40::new("top"), &vector, &vector, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                SoundModule::play_se(boma, Hash40::new("se_common_spirits_machstamp_landing"), true, false, false, false, enSEType(0));
                STALL_TIMER[ENTRY_ID] = 721;
            }
            else if STALL_TIMER[ENTRY_ID] == 721 {
                DamageModule::add_damage(boma, 0.5, 0);
                if DamageModule::damage(boma, 0) >= 200.0 {
                    STALL_TIMER[ENTRY_ID] = 0;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
                }
            }
            else {
                STALL_TIMER[ENTRY_ID] += 1;
            }
        }
        else {
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEMO {
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
        if DamageModule::reaction(boma, 0) > 1.0 { 
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
    }
}

pub fn install() {
    Agent::new("fighter")
        .on_line(Main, global_fighter_frame)
        .install();

    skyline::install_hooks!(
        is_valid_just_shield_reflector,
        normal_weapon_hit_handler
    );
}
