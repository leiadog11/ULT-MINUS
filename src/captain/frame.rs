use super::*;

// OPFF
pub unsafe extern "C" fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if motion_kind != hash40("attack_hi3") && motion_kind != hash40("attack_100") && motion_kind != hash40("attack_air_f")  {
            if MotionModule::is_end(fighter.module_accessor) && situation_kind == *SITUATION_KIND_GROUND {
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_FALL || status_kind == *FIGHTER_STATUS_KIND_WAIT {
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // CLEAR FIRE BIRD ON F SMASH DAMAGE
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        // GUN COOLDOWN
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN);
        }

        // GROUND CHECK FOR UP B COUNT
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT);
        } 
    }
}

// ON START
pub unsafe extern "C" fn captain_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GUN_COOLDOWN);
        WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT);
    }
}

pub fn install() {
    Agent::new("captain")
        .on_line(Main, captain_frame)
        .on_start(captain_start)
        .install();
}