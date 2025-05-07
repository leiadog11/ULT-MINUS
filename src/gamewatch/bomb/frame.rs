use super::*;

static mut exploded: bool = false;

// OPFF
pub unsafe extern "C" fn bomb_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = get_entry_id(owner_boma);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        let status_kind = StatusModule::status_kind(weapon.module_accessor);

        if motion_kind == hash40("fly") { 
            if MotionModule::frame(weapon.module_accessor) == 1.0 {
                BOMB_OUT[ENTRY_ID] = true;
                exploded = false;
            }

            let b1x = PostureModule::pos_x(weapon.module_accessor);
            let b1y = PostureModule::pos_y(weapon.module_accessor);
            
            let opponent_bomas = get_opponent_bomas(owner_boma);
            for opponent_boma in opponent_bomas.iter() {
                let b2x = PostureModule::pos_x(*opponent_boma);
                let b2y = PostureModule::pos_y(*opponent_boma);   
    
                // distance formula
                let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
                let d = dSquared.sqrt();
    
                if d < 22.0 && !exploded {
                    exploded = true;
                    BOMB_OUT[ENTRY_ID] = false;
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }

        // RESET BOMB FLAG
        if motion_kind == hash40("burst") { 
            BOMB_OUT[ENTRY_ID] = false;
        }
        if WorkModule::is_flag(weapon.module_accessor, *WEAPON_GAMEWATCH_BOMB_STATUS_WORK_FLAG_DAMAGE) {
            BOMB_OUT[ENTRY_ID] = false;
        }
    }
}

pub fn install() {
    Agent::new("gamewatch_bomb")
        .on_line(Main, bomb_frame)
        .install();
}