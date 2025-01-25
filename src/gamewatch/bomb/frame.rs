use super::*;

static mut exploded: bool = false;

// OPFF
pub unsafe extern "C" fn bomb_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        static mut opp_bomas: Option<Vec<*mut BattleObjectModuleAccessor>> = None;
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        let status_kind = StatusModule::status_kind(weapon.module_accessor);

        if motion_kind == hash40("fly") { 
            if MotionModule::frame(weapon.module_accessor) == 1.0 {
                WorkModule::on_flag(owner_boma, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT);
                exploded = false;
            }
            let entry_count = lua_bind::FighterManager::entry_count(singletons::FighterManager());
            let entry_count_usize = entry_count as usize;
            let mut opponent_bomas: Vec<*mut BattleObjectModuleAccessor> = Vec::with_capacity(entry_count_usize);
            let mut boma_counter = 0;
            
            for _ in 0..entry_count_usize { 
                let curr_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(boma_counter));
                if curr_boma == owner_boma {
                }
                else {
                    opponent_bomas.push(sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(boma_counter)));
                }
                boma_counter += 1;
            }

            let b1x = PostureModule::pos_x(weapon.module_accessor);
            let b1y = PostureModule::pos_y(weapon.module_accessor);

            for &boma_ptr in &opponent_bomas {
                let b2x = PostureModule::pos_x(boma_ptr);
                let b2y = PostureModule::pos_y(boma_ptr);   
    
                // distance formula
                let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
                let d = dSquared.sqrt();
    
                if d < 21.0 && !exploded {
                    exploded = true;
                    WorkModule::off_flag(owner_boma, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT);
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }

        // RESET BOMB FLAG
        if motion_kind == hash40("burst") { 
            WorkModule::off_flag(owner_boma, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT);
        }
        if status_kind == *WEAPON_GAMEWATCH_BOMB_STATUS_WORK_FLAG_DAMAGE {
            WorkModule::off_flag(owner_boma, FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_BOMB_OUT);
        }
    }
}

pub fn install() {
    Agent::new("gamewatch_bomb")
        .on_line(Main, bomb_frame)
        .install();
}