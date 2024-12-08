use super::*;

static mut exploded: bool = false;

// OPFF
pub unsafe extern "C" fn bomb_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        static mut opp_bomas: Option<Vec<*mut BattleObjectModuleAccessor>> = None;
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);

        if motion_kind == hash40("fly") { 
            if MotionModule::frame(weapon.module_accessor) == 1.0 {
                exploded = false;
                OPPONENT_BOMAS = Some(get_opponent_bomas_weapon(owner_boma));
                opp_bomas = OPPONENT_BOMAS.clone();
            }
        }
        
        let b1x = PostureModule::pos_x(weapon.module_accessor);
        let b1y = PostureModule::pos_y(weapon.module_accessor);

        if let Some(ref opponent_bomas) = opp_bomas {
            for (index, &boma_ptr) in opponent_bomas.iter().enumerate() {
                let b2x = PostureModule::pos_x(boma_ptr);
                let b2y = PostureModule::pos_y(boma_ptr);   
    
                // distance formula
                let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
                let d = dSquared.sqrt();
    
                if d < 21.0 && !exploded {
                    exploded = true;
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        } 
    }
}

pub fn install() {
    Agent::new("gamewatch_bomb")
        .on_line(Main, bomb_frame)
        .install();
}