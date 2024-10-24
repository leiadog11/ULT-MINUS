
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

static mut INT_OFFSET : usize = 0x4ded80;
static mut FLOAT_OFFSET : usize = 0x4dedc0;

//CHANGE JUMP SQUAT TO 2
#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook(
boma: u64,
param_type: u64,
param_hash: u64) -> i32 {
    let ret = original!()(boma, param_type, param_hash);
	let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
		
        //ALL
		if fighter_kind == FIGHTER_KIND_ALL {
				if param_type == hash40("jump_squat_frame") {
					return 2;
				} 
		}

        //LUIGI
        if fighter_kind == FIGHTER_KIND_LUIGI {
            if param_type == hash40("wall_jump_type") {
                return 1;
            }  
        }
	
	}
ret
}


//Parry Reflects
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    return true;
}

// Use this for general per-frame fighter-level hooks
/*
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if StatusModule::is_situation_changed(module_accessor) {
            let situation_kind = &format!("{}", StatusModule::situation_kind(module_accessor));
            println!(
                "[Fighter Hook]\nFighter Kind: {}\nStatus Kind: {}\nSituation Kind: {}",
                utility::get_kind(module_accessor),
                StatusModule::status_kind(module_accessor),
                match StatusModule::situation_kind(module_accessor) {
                    0 => "SITUATION_KIND_GROUND",
                    1 => "SITUATION_KIND_CLIFF",
                    2 => "SITUATION_KIND_AIR",
                    _ => situation_kind
                }
            );
        }
    }
}


// Use this for general per-frame weapon-level hooks
#[weapon_frame_callback]
pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}
*/

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

pub fn install() {
    /*
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        global_weapon_frame
    );
    */
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        is_valid_just_shield_reflector,
        int_param_accessor_hook,
        );
}
